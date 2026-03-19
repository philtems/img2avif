use crate::error::{ConversionError, Result};
use crate::config::{Config, ColorSpaceArg, BitDepthArg};
use image::{DynamicImage, ImageReader};
use ravif::{Encoder, Img, ColorModel, BitDepth};
use rgb::{RGB8, RGBA8};
use std::path::Path;
use std::fs;

pub struct Converter {
    quality: f32,
    alpha_quality: f32,
    speed: u8,
    color_space: ColorSpaceArg,
    bit_depth: BitDepthArg,
    lossless: bool,
    discard_if_larger: bool,
    keep_metadata: bool,
    quiet: bool,
    verbose: bool,
}

impl Converter {
    pub fn from_config(config: &Config) -> Self {
        Self {
            quality: config.quality,
            alpha_quality: config.alpha_quality,
            speed: config.speed,
            color_space: config.color_space,
            bit_depth: config.bit_depth,
            lossless: config.lossless,
            discard_if_larger: config.discard_if_larger,
            keep_metadata: config.keep_metadata,
            quiet: config.quiet,
            verbose: config.verbose,
        }
    }

    pub fn load_image(&self, path: &Path) -> Result<DynamicImage> {
        if self.verbose {
            println!("   Loading: {}", path.display());
        }
        
        let img = ImageReader::open(path)
            .map_err(ConversionError::Io)?
            .with_guessed_format()
            .map_err(|e| ConversionError::ImageDecode(e.to_string()))?
            .decode()
            .map_err(|e| ConversionError::ImageDecode(e.to_string()))?;
        
        Ok(img)
    }

    fn get_file_size(&self, path: &Path) -> Result<u64> {
        fs::metadata(path)
            .map(|m| m.len())
            .map_err(ConversionError::Io)
    }

    pub fn configure_encoder(&self) -> Encoder {
        let mut encoder = Encoder::new();
        
        if self.lossless {
            encoder = encoder.with_quality(100.0);
        } else {
            encoder = encoder.with_quality(self.quality);
        }
        
        encoder = encoder
            .with_alpha_quality(self.alpha_quality)
            .with_speed(self.speed)
            .with_internal_color_model(ColorModel::RGB);
        
        encoder = match self.bit_depth {
            BitDepthArg::Bit8 => encoder.with_bit_depth(BitDepth::Eight),
            BitDepthArg::Bit10 => encoder.with_bit_depth(BitDepth::Ten),
            BitDepthArg::Bit12 => encoder,
            BitDepthArg::Auto => encoder,
        };
        
        encoder
    }

    pub fn convert(&self, input_path: &Path, output_path: &Path) -> Result<u64> {
        let img = self.load_image(input_path)?;
        
        if self.verbose {
            println!("   Dimensions: {}x{}", img.width(), img.height());
            println!("   Color mode: {:?}", img.color());
            println!("   Quality: {:.1}", self.quality);
            if self.lossless {
                println!("   Mode: lossless");
            }
        }

        let encoder = self.configure_encoder();
        
        let encoded = if img.color().has_alpha() {
            if self.verbose {
                println!("   Type: RGBA (with transparency)");
            }
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            let pixels: &[RGBA8] = bytemuck::cast_slice(rgba.as_raw());
            let ravif_img = Img::new(pixels, width as usize, height as usize);
            
            encoder.encode_rgba(ravif_img)
                .map_err(|e| ConversionError::AvifEncode(e.to_string()))?
        } else {
            if self.verbose {
                println!("   Type: RGB (no transparency)");
            }
            let rgb = img.to_rgb8();
            let (width, height) = rgb.dimensions();
            let pixels: &[RGB8] = bytemuck::cast_slice(rgb.as_raw());
            let ravif_img = Img::new(pixels, width as usize, height as usize);
            
            encoder.encode_rgb(ravif_img)
                .map_err(|e| ConversionError::AvifEncode(e.to_string()))?
        };

        if self.discard_if_larger {
            let original_size = self.get_file_size(input_path)?;
            let avif_size = encoded.avif_file.len() as u64;
            
            if avif_size > original_size {
                if self.verbose {
                    println!("   Skipping: AVIF larger than original ({:.2} KB > {:.2} KB)", 
                        avif_size as f64 / 1024.0, original_size as f64 / 1024.0);
                }
                return Ok(0);
            }
        }

        fs::write(output_path, &encoded.avif_file)
            .map_err(ConversionError::Io)?;

        let avif_size = encoded.avif_file.len() as u64;
        
        if !self.quiet {
            let original_size = self.get_file_size(input_path).unwrap_or(0);
            let ratio = if original_size > 0 {
                (avif_size as f64 / original_size as f64 * 100.0 * 100.0).round() / 100.0
            } else {
                0.0
            };
            
            println!("   ✓ Converted: {} -> {} ({:.1} KB, {:.1}% of original)", 
                input_path.file_name().unwrap_or_default().to_string_lossy(),
                output_path.file_name().unwrap_or_default().to_string_lossy(),
                avif_size as f64 / 1024.0, ratio);
        }

        Ok(avif_size)
    }

    pub fn convert_from_image(&self, img: &DynamicImage, output_path: &Path, verbose: bool) -> Result<u64> {
        if verbose {
            println!("   Dimensions: {}x{}", img.width(), img.height());
            println!("   Color mode: {:?}", img.color());
        }

        let encoder = self.configure_encoder();
        
        let encoded = if img.color().has_alpha() {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            let pixels: &[RGBA8] = bytemuck::cast_slice(rgba.as_raw());
            let ravif_img = Img::new(pixels, width as usize, height as usize);
            
            encoder.encode_rgba(ravif_img)
                .map_err(|e| ConversionError::AvifEncode(e.to_string()))?
        } else {
            let rgb = img.to_rgb8();
            let (width, height) = rgb.dimensions();
            let pixels: &[RGB8] = bytemuck::cast_slice(rgb.as_raw());
            let ravif_img = Img::new(pixels, width as usize, height as usize);
            
            encoder.encode_rgb(ravif_img)
                .map_err(|e| ConversionError::AvifEncode(e.to_string()))?
        };

        fs::write(output_path, &encoded.avif_file)
            .map_err(ConversionError::Io)?;

        Ok(encoded.avif_file.len() as u64)
    }
}

