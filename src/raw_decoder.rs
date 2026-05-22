use crate::error::{ConversionError, Result};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::path::Path;
use rayon::prelude::*;

pub struct RawDecoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BayerPattern {
    RGGB, BGGR, GRBG, GBRG,
}

impl RawDecoder {
    pub fn is_raw_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            matches!(ext_lower.as_str(),
                "nef" | "nrw" | "cr2" | "cr3" | "crw" | "arw" | "srf" | "sr2" |
                "raf" | "orf" | "rw2" | "pef" | "x3f" | "dng" | "raw" | "rwl" |
                "mrw" | "dcr" | "kdc" | "3fr" | "ari" | "bay" | "cap" | "data" |
                "dcs" | "drf" | "erf" | "fff" | "iiq" | "mef" | "mos" | "mdc" |
                "obm" | "ptx" | "pxn" | "qtk" | "rdc" | "srw" | "sti" | "rwz")
        } else {
            false
        }
    }

    pub fn get_raw_orientation(path: &Path) -> Result<u16> {
        let raw_file = rawloader::decode_file(path)
            .map_err(|e| ConversionError::RawDecode(format!("{}", e)))?;
        
        Ok(match raw_file.orientation {
            rawloader::Orientation::Normal => 1,
            rawloader::Orientation::HorizontalFlip => 2,
            rawloader::Orientation::Rotate180 => 3,
            rawloader::Orientation::VerticalFlip => 4,
            rawloader::Orientation::Transpose => 5,
            rawloader::Orientation::Rotate90 => 6,
            rawloader::Orientation::Transverse => 7,
            rawloader::Orientation::Rotate270 => 8,
            rawloader::Orientation::Unknown => 1,
        })
    }

    /// Affiche les métadonnées détaillées du fichier RAW
    pub fn print_metadata(raw: &rawloader::RawImage, verbose: bool) {
        if !verbose { return; }
        
        println!("   📷 RAW Metadata:");
        println!("      Camera: {} {}", raw.make, raw.model);
        println!("      Resolution: {} x {} pixels", raw.width, raw.height);
        println!("      CFA Pattern: {}", raw.cfa.to_string());
        
        let wb = if raw.wb_coeffs[1].abs() > 0.0001 {
            [raw.wb_coeffs[0] / raw.wb_coeffs[1], 1.0, raw.wb_coeffs[2] / raw.wb_coeffs[1]]
        } else {
            [1.0, 1.0, 1.0]
        };
        println!("      White Balance: R={:.3}, G={:.3}, B={:.3}", wb[0], wb[1], wb[2]);
        println!("      Black Level: {}", raw.blacklevels[0]);
        println!("      White Level: {}", raw.whitelevels[0]);
        println!("      Orientation: {:?}", raw.orientation);
        println!("      Camera (clean): {} {}", raw.clean_make, raw.clean_model);
    }

    /// Calcule l'exposition automatique basée sur l'histogramme
    fn calculate_auto_exposure(data: &[u16], black_level: f32, dynamic_range: f32, target_percentile: f32) -> f32 {
        let step = (data.len() / 10000).max(1);
        
        let mut samples: Vec<f32> = data.iter()
            .step_by(step)
            .map(|&p| ((p as f32 - black_level) / dynamic_range).clamp(0.0, 1.0))
            .filter(|&v| v > 0.01)
            .collect();
        
        if samples.is_empty() {
            return 1.0;
        }
        
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let percentile_idx = ((samples.len() - 1) as f32 * target_percentile) as usize;
        let max_sample = samples[percentile_idx];
        
        let exposure = (0.95 / max_sample.max(0.05)).clamp(0.5, 5.0);
        
        exposure
    }

    /// Calcule la saturation automatique basée sur l'exposition
    fn calculate_auto_saturation(exposure: f32) -> f32 {
        // Saturation = 1.0 + (exposure - 1.0) * 0.2
        // Limité entre 0.8 et 2.0
        let saturation = 1.0 + (exposure - 1.0) * 0.2;
        saturation.clamp(0.8, 2.0)
    }

    /// Applique la saturation à un pixel
    fn apply_saturation(r: f32, g: f32, b: f32, saturation: f32) -> (f32, f32, f32) {
        if saturation == 1.0 {
            return (r, g, b);
        }
        
        // Luminance (standard ITU-R BT.601)
        let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
        
        let r_sat = luminance + (r - luminance) * saturation;
        let g_sat = luminance + (g - luminance) * saturation;
        let b_sat = luminance + (b - luminance) * saturation;
        
        (r_sat.clamp(0.0, 1.0), g_sat.clamp(0.0, 1.0), b_sat.clamp(0.0, 1.0))
    }

    /// Applique le contraste à un pixel
    fn apply_contrast(c: f32, contrast: f32) -> f32 {
        if contrast == 1.0 {
            return c;
        }
        
        let adjusted = (c - 0.5) * contrast + 0.5;
        adjusted.clamp(0.0, 1.0)
    }

    pub fn decode_raw_with_options(
        path: &Path, 
        exposure_opt: &str, 
        percentile: f32,
        saturation_opt: &str,
        contrast: f32,
        verbose: bool
    ) -> Result<DynamicImage> {
        let raw_file = rawloader::decode_file(path)
            .map_err(|e| ConversionError::RawDecode(format!("Failed to decode RAW: {}", e)))?;
        
        Self::print_metadata(&raw_file, verbose);
        
        let width = raw_file.width as u32;
        let height = raw_file.height as u32;
        
        let data = match raw_file.data {
            rawloader::RawImageData::Integer(data) => data,
            _ => return Err(ConversionError::RawDecode("Float RAW data not supported".to_string())),
        };
        
        let pattern = match raw_file.cfa.to_string().as_str() {
            "RGGB" => BayerPattern::RGGB,
            "BGGR" => BayerPattern::BGGR,
            "GRBG" => BayerPattern::GRBG,
            "GBRG" => BayerPattern::GBRG,
            _ => BayerPattern::RGGB,
        };
        
        let wb_coeffs = raw_file.wb_coeffs;
        let wb = if wb_coeffs[1].abs() > 0.0001 {
            [wb_coeffs[0] / wb_coeffs[1], 1.0, wb_coeffs[2] / wb_coeffs[1]]
        } else {
            [1.0, 1.0, 1.0]
        };
        
        let max_value = raw_file.whitelevels[0] as f32;
        let black_level = raw_file.blacklevels[0] as f32;
        let dynamic_range = (max_value - black_level).max(1.0);
        
        // Calculer l'exposition
        let exposure = if exposure_opt == "auto" {
            let auto_exp = Self::calculate_auto_exposure(&data, black_level, dynamic_range, percentile);
            if verbose {
                println!("      Auto-exposure: {:.2}x (target percentile: {:.0}%)", auto_exp, percentile * 100.0);
            }
            auto_exp
        } else {
            let manual_exp = exposure_opt.parse::<f32>().unwrap_or(1.0);
            if verbose {
                println!("      Manual exposure: {:.2}x", manual_exp);
            }
            manual_exp.clamp(0.5, 5.0)
        };
        
        // Calculer la saturation
        let saturation = if saturation_opt == "auto" {
            let auto_sat = Self::calculate_auto_saturation(exposure);
            if verbose {
                println!("      Auto-saturation: {:.2}x (based on exposure)", auto_sat);
            }
            auto_sat
        } else {
            let manual_sat = saturation_opt.parse::<f32>().unwrap_or(1.0);
            if verbose {
                println!("      Manual saturation: {:.2}x", manual_sat);
            }
            manual_sat.clamp(0.5, 2.0)
        };
        
        if verbose {
            println!("      Contrast: {:.2}x", contrast);
            println!("      Processing {}x{} image...", width, height);
        }
        
        // Démosaïcage et conversion
        let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        let buffer = img_buffer.as_mut();
        
        buffer.par_chunks_mut((width * 3) as usize)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let (r_raw, g_raw, b_raw) = demosaic_pixel(
                        &data, x, y as u32, width, height, pattern
                    );
                    
                    let r_bl = (r_raw - black_level).max(0.0);
                    let g_bl = (g_raw - black_level).max(0.0);
                    let b_bl = (b_raw - black_level).max(0.0);
                    
                    let r_wb = r_bl * wb[0];
                    let g_wb = g_bl * wb[1];
                    let b_wb = b_bl * wb[2];
                    
                    let r_norm = ((r_wb / dynamic_range) * exposure).clamp(0.0, 1.0);
                    let g_norm = ((g_wb / dynamic_range) * exposure).clamp(0.0, 1.0);
                    let b_norm = ((b_wb / dynamic_range) * exposure).clamp(0.0, 1.0);
                    
                    // Appliquer la saturation
                    let (r_sat, g_sat, b_sat) = Self::apply_saturation(r_norm, g_norm, b_norm, saturation);
                    
                    // Appliquer le contraste
                    let r_cont = Self::apply_contrast(r_sat, contrast);
                    let g_cont = Self::apply_contrast(g_sat, contrast);
                    let b_cont = Self::apply_contrast(b_sat, contrast);
                    
                    // Correction gamma
                    let r_gamma = r_cont.powf(1.0 / 2.2);
                    let g_gamma = g_cont.powf(1.0 / 2.2);
                    let b_gamma = b_cont.powf(1.0 / 2.2);
                    
                    let base = (x * 3) as usize;
                    row[base + 0] = (r_gamma * 255.0) as u8;
                    row[base + 1] = (g_gamma * 255.0) as u8;
                    row[base + 2] = (b_gamma * 255.0) as u8;
                }
            });
        
        Ok(DynamicImage::ImageRgb8(img_buffer))
    }
}

// Fonction de démosaïcage (identique à avant)
fn demosaic_pixel(
    raw_data: &[u16], 
    x: u32, 
    y: u32, 
    width: u32, 
    height: u32,
    pattern: BayerPattern
) -> (f32, f32, f32) {
    let x_i = x as i32;
    let y_i = y as i32;
    
    let get_raw = |px: i32, py: i32| -> f32 {
        let clamped_x = px.max(0).min(width as i32 - 1) as u32;
        let clamped_y = py.max(0).min(height as i32 - 1) as u32;
        raw_data[(clamped_y * width + clamped_x) as usize] as f32
    };
    
    let get_color = |px: i32, py: i32| -> char {
        let (row_even, col_even) = (py % 2 == 0, px % 2 == 0);
        match pattern {
            BayerPattern::RGGB => {
                if row_even { if col_even { 'R' } else { 'G' } }
                else { if col_even { 'G' } else { 'B' } }
            },
            BayerPattern::BGGR => {
                if row_even { if col_even { 'B' } else { 'G' } }
                else { if col_even { 'G' } else { 'R' } }
            },
            BayerPattern::GRBG => {
                if row_even { if col_even { 'G' } else { 'R' } }
                else { if col_even { 'B' } else { 'G' } }
            },
            BayerPattern::GBRG => {
                if row_even { if col_even { 'G' } else { 'B' } }
                else { if col_even { 'R' } else { 'G' } }
            },
        }
    };
    
    match get_color(x_i, y_i) {
        'R' => {
            let r = get_raw(x_i, y_i);
            let b = (get_raw(x_i - 1, y_i - 1) + get_raw(x_i + 1, y_i - 1) +
                     get_raw(x_i - 1, y_i + 1) + get_raw(x_i + 1, y_i + 1)) / 4.0;
            let g_n = get_raw(x_i, y_i - 1);
            let g_s = get_raw(x_i, y_i + 1);
            let g_w = get_raw(x_i - 1, y_i);
            let g_e = get_raw(x_i + 1, y_i);
            let grad_v = (g_n - g_s).abs();
            let grad_h = (g_w - g_e).abs();
            let g = if grad_v < grad_h {
                (g_n + g_s) / 2.0
            } else if grad_h < grad_v {
                (g_w + g_e) / 2.0
            } else {
                (g_n + g_s + g_w + g_e) / 4.0
            };
            (r, g, b)
        },
        'B' => {
            let b = get_raw(x_i, y_i);
            let r = (get_raw(x_i - 1, y_i - 1) + get_raw(x_i + 1, y_i - 1) +
                     get_raw(x_i - 1, y_i + 1) + get_raw(x_i + 1, y_i + 1)) / 4.0;
            let g_n = get_raw(x_i, y_i - 1);
            let g_s = get_raw(x_i, y_i + 1);
            let g_w = get_raw(x_i - 1, y_i);
            let g_e = get_raw(x_i + 1, y_i);
            let grad_v = (g_n - g_s).abs();
            let grad_h = (g_w - g_e).abs();
            let g = if grad_v < grad_h {
                (g_n + g_s) / 2.0
            } else if grad_h < grad_v {
                (g_w + g_e) / 2.0
            } else {
                (g_n + g_s + g_w + g_e) / 4.0
            };
            (r, g, b)
        },
        'G' => {
            let g = get_raw(x_i, y_i);
            let is_red_row = match pattern {
                BayerPattern::RGGB => y_i % 2 == 0,
                BayerPattern::BGGR => y_i % 2 == 1,
                BayerPattern::GRBG => y_i % 2 == 0,
                BayerPattern::GBRG => y_i % 2 == 1,
            };
            
            if is_red_row {
                let r = (get_raw(x_i - 1, y_i) + get_raw(x_i + 1, y_i)) / 2.0;
                let b = (get_raw(x_i, y_i - 1) + get_raw(x_i, y_i + 1)) / 2.0;
                (r, g, b)
            } else {
                let b = (get_raw(x_i - 1, y_i) + get_raw(x_i + 1, y_i)) / 2.0;
                let r = (get_raw(x_i, y_i - 1) + get_raw(x_i, y_i + 1)) / 2.0;
                (r, g, b)
            }
        },
        _ => (0.0, 0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raw_file_detection() {
        assert!(RawDecoder::is_raw_file(Path::new("photo.nef")));
        assert!(RawDecoder::is_raw_file(Path::new("photo.CR2")));
        assert!(RawDecoder::is_raw_file(Path::new("photo.dng")));
        assert!(!RawDecoder::is_raw_file(Path::new("photo.jpg")));
        assert!(!RawDecoder::is_raw_file(Path::new("photo.png")));
    }
}

