use crate::error::{ConversionError, Result};
use crate::config::RawParams;
use image::{DynamicImage, ImageBuffer, Rgb};
use std::path::Path;

pub struct RawDecoder;

impl RawDecoder {
    /// Vérifie si un fichier est un format RAW supporté
    pub fn is_raw_file(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            match ext_lower.as_str() {
                "nef" | "nrw" | "cr2" | "cr3" | "crw" | "arw" | "srf" | "sr2" |
                "raf" | "orf" | "rw2" | "pef" | "x3f" | "dng" | "raw" | "rwl" |
                "mrw" | "dcr" | "kdc" | "3fr" | "ari" | "bay" | "cap" | "data" |
                "dcs" | "drf" | "erf" | "fff" | "iiq" | "mef" | "mos" | "mdc" |
                "obm" | "ptx" | "pxn" | "qtk" | "rdc" | "srw" | "sti" | "rwz" => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Récupère l'orientation EXIF d'un fichier RAW
    pub fn get_raw_orientation(path: &Path) -> Result<u16> {
        let raw_image = rawloader::decode_file(path)
            .map_err(|e| ConversionError::RawDecode(format!("{}", e)))?;
        
        match raw_image.orientation {
            rawloader::Orientation::Normal => Ok(1),
            rawloader::Orientation::HorizontalFlip => Ok(2),
            rawloader::Orientation::Rotate180 => Ok(3),
            rawloader::Orientation::VerticalFlip => Ok(4),
            rawloader::Orientation::Transpose => Ok(5),
            rawloader::Orientation::Rotate90 => Ok(6),
            rawloader::Orientation::Transverse => Ok(7),
            rawloader::Orientation::Rotate270 => Ok(8),
            rawloader::Orientation::Unknown => Ok(1),
        }
    }

    /// Charge et décode un fichier RAW avec les paramètres par défaut
    pub fn decode_raw(path: &Path) -> Result<DynamicImage> {
        let default_params = RawParams::default();
        Self::decode_raw_with_params(path, &default_params)
    }

    /// Charge et décode un fichier RAW avec paramètres configurables
    pub fn decode_raw_with_params(path: &Path, params: &RawParams) -> Result<DynamicImage> {
        let raw_image = rawloader::decode_file(path)
            .map_err(|e| ConversionError::RawDecode(format!("{}", e)))?;
        
        Self::rawimage_to_dynamic_image_with_params(&raw_image, params)
    }

    /// Convertit un RawImage en DynamicImage avec paramètres
    fn rawimage_to_dynamic_image_with_params(raw: &rawloader::RawImage, params: &RawParams) -> Result<DynamicImage> {
        let width = raw.width;
        let height = raw.height;
        
        let raw_data = match &raw.data {
            rawloader::RawImageData::Integer(data) => data,
            rawloader::RawImageData::Float(_) => {
                return Err(ConversionError::RawDecode(
                    "Float RAW data not supported for conversion".to_string()
                ));
            }
        };
        
        let mut rgb_buffer = ImageBuffer::new(width as u32, height as u32);
        
        let black_level = raw.blacklevels[0] as f64;
        let white_level = raw.whitelevels[0] as f64;
        
        // Balance des blancs depuis wb_coeffs (avec température si spécifiée)
        let (red_wb, green_wb, blue_wb) = if params.temperature > 0.0 {
            Self::temperature_to_wb(params.temperature)
        } else {
            Self::get_white_balance(raw)
        };
        
        // Exposition (combine exposition auto + compensation utilisateur)
        let auto_exposure = Self::calculate_exposure(raw, raw_data, black_level, white_level, params.highlight);
        let exposure_comp = auto_exposure * params.exposure;
        
        let cfa = &raw.cfa;
        
        for y in 0..height {
            for x in 0..width {
                let (r_raw, g_raw, b_raw) = Self::demosaic_with_wb(x, y, raw, raw_data, cfa, red_wb, green_wb, blue_wb);
                
                let r_exp = (r_raw * exposure_comp).clamp(0.0, 1.0);
                let g_exp = (g_raw * exposure_comp).clamp(0.0, 1.0);
                let b_exp = (b_raw * exposure_comp).clamp(0.0, 1.0);
                
                let (r_sat, g_sat, b_sat) = Self::apply_saturation(r_exp, g_exp, b_exp, params.saturation);
                
                let r_gamma = Self::gamma_custom(r_sat, params.gamma);
                let g_gamma = Self::gamma_custom(g_sat, params.gamma);
                let b_gamma = Self::gamma_custom(b_sat, params.gamma);
                
                let r_contrast = Self::apply_contrast(r_gamma, params.contrast);
                let g_contrast = Self::apply_contrast(g_gamma, params.contrast);
                let b_contrast = Self::apply_contrast(b_gamma, params.contrast);
                
                let r8 = (r_contrast * 255.0) as u8;
                let g8 = (g_contrast * 255.0) as u8;
                let b8 = (b_contrast * 255.0) as u8;
                
                rgb_buffer.put_pixel(x as u32, y as u32, Rgb([r8, g8, b8]));
            }
        }
        
        Ok(DynamicImage::ImageRgb8(rgb_buffer))
    }

    /// Convertit une température de couleur en coefficients de balance des blancs
    fn temperature_to_wb(temperature_k: f64) -> (f64, f64, f64) {
        let temp = temperature_k / 1000.0;
        
        let red = if temp <= 66.0 {
            1.0
        } else {
            1.0 + 0.2 * (temp - 66.0).powf(0.5)
        };
        
        let blue = if temp <= 66.0 {
            if temp <= 20.0 {
                0.8 + 0.2 * (temp / 20.0)
            } else {
                1.0 + 0.1 * ((temp - 20.0) / 46.0)
            }
        } else {
            1.0
        };
        
        let green = 1.0;
        (red, green, blue)
    }

    /// Récupère les coefficients de balance des blancs depuis les métadonnées
    fn get_white_balance(raw: &rawloader::RawImage) -> (f64, f64, f64) {
        let mut red = raw.wb_coeffs[0] as f64;
        let mut green = raw.wb_coeffs[1] as f64;
        let mut blue = raw.wb_coeffs[2] as f64;
        
        if red <= 0.0 || red.is_nan() { red = 1.0; }
        if green <= 0.0 || green.is_nan() { green = 1.0; }
        if blue <= 0.0 || blue.is_nan() { blue = 1.0; }
        
        let green_inv = 1.0 / green;
        (red * green_inv, 1.0, blue * green_inv)
    }

    /// Calcule l'exposition automatique basée sur les hautes lumières
    fn calculate_exposure(raw: &rawloader::RawImage, raw_data: &[u16], black_level: f64, white_level: f64, target: f64) -> f64 {
        let mut samples = Vec::new();
        let step = (raw.width.max(64) / 64).max(1);
        
        for y in (0..raw.height).step_by(step) {
            for x in (0..raw.width).step_by(step) {
                let idx = (y * raw.width + x) as usize;
                if idx < raw_data.len() {
                    let val = (raw_data[idx] as f64 - black_level) / (white_level - black_level);
                    samples.push(val.clamp(0.0, 1.0));
                }
            }
        }
        
        if samples.is_empty() {
            return 1.0;
        }
        
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let percentile_95 = samples[samples.len() * 95 / 100];
        
        let exposure = target / percentile_95.max(0.01);
        exposure.clamp(0.5, 2.0)
    }

    /// Démosaïcage avec balance des blancs
    fn demosaic_with_wb(x: usize, y: usize, raw: &rawloader::RawImage, raw_data: &[u16], 
                       cfa: &rawloader::CFA, red_wb: f64, green_wb: f64, blue_wb: f64) -> (f64, f64, f64) {
        let px = x as i32;
        let py = y as i32;
        let width = raw.width as i32;
        let height = raw.height as i32;
        
        let black_level = raw.blacklevels[0] as f64;
        let dynamic_range = (raw.whitelevels[0] - raw.blacklevels[0]) as f64;
        
        let idx = (y * raw.width + x) as usize;
        let current_val = if idx < raw_data.len() {
            (raw_data[idx] as f64 - black_level) / dynamic_range
        } else {
            0.0
        };
        let current_val = current_val.clamp(0.0, 1.0);
        
        let current_channel = cfa.color_at(y, x);
        
        let mut red_samples = Vec::new();
        let mut green_samples = Vec::new();
        let mut blue_samples = Vec::new();
        
        for dy in -2..=2 {
            for dx in -2..=2 {
                let nx = px + dx;
                let ny = py + dy;
                
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    let nidx = (ny as usize * raw.width + nx as usize) as usize;
                    if let Some(&neighbor_val) = raw_data.get(nidx) {
                        let nval = (neighbor_val as f64 - black_level) / dynamic_range;
                        let nval = nval.clamp(0.0, 1.0);
                        let channel = cfa.color_at(ny as usize, nx as usize);
                        
                        let distance = ((dx*dx + dy*dy) as f64).sqrt();
                        let weight = (-distance / 1.5).exp();
                        
                        match channel {
                            0 => red_samples.push((nval * weight, weight)),
                            1 => green_samples.push((nval * weight, weight)),
                            2 => blue_samples.push((nval * weight, weight)),
                            _ => {}
                        }
                    }
                }
            }
        }
        
        let red = Self::weighted_average(&red_samples);
        let green = Self::weighted_average(&green_samples);
        let blue = Self::weighted_average(&blue_samples);
        
        match current_channel {
            0 => (current_val * red_wb, green * green_wb, blue * blue_wb),
            1 => (red * red_wb, current_val * green_wb, blue * blue_wb),
            2 => (red * red_wb, green * green_wb, current_val * blue_wb),
            _ => (red * red_wb, green * green_wb, blue * blue_wb),
        }
    }

    /// Calcule la moyenne pondérée
    fn weighted_average(samples: &[(f64, f64)]) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_weighted: f64 = samples.iter().map(|(val, weight)| val * weight).sum();
        let sum_weights: f64 = samples.iter().map(|(_, weight)| weight).sum();
        if sum_weights > 0.0 { sum_weighted / sum_weights } else { 0.0 }
    }

    /// Applique une augmentation de saturation
    fn apply_saturation(r: f64, g: f64, b: f64, saturation: f64) -> (f64, f64, f64) {
        let gray = (r + g + b) / 3.0;
        let r_sat = gray + (r - gray) * saturation;
        let g_sat = gray + (g - gray) * saturation;
        let b_sat = gray + (b - gray) * saturation;
        (r_sat.clamp(0.0, 1.0), g_sat.clamp(0.0, 1.0), b_sat.clamp(0.0, 1.0))
    }

    /// Applique une courbe de contraste
    fn apply_contrast(c: f64, contrast: f64) -> f64 {
        let adjusted = (c - 0.5) * contrast + 0.5;
        adjusted.clamp(0.0, 1.0)
    }

    /// Gamma personnalisé
    fn gamma_custom(c: f64, gamma: f64) -> f64 {
        if c <= 0.0 {
            0.0
        } else {
            c.powf(1.0 / gamma)
        }
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

