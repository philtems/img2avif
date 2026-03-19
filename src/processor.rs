use crate::error::{ConversionError, Result};
use crate::config::Config;
use crate::converter::Converter;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};

pub struct ImageProcessor {
    config: Config,
    converter: Converter,
    supported_extensions: Vec<String>,
}

impl ImageProcessor {
    pub fn new(config: Config) -> Self {
        let converter = Converter::from_config(&config);
        
        let supported_extensions = vec![
            "jpg".to_string(), "jpeg".to_string(), "png".to_string(), 
            "bmp".to_string(), "gif".to_string(), "tiff".to_string(), 
            "tif".to_string(), "webp".to_string(), "ico".to_string()
        ];
        
        Self {
            config,
            converter,
            supported_extensions,
        }
    }

    fn is_image_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            self.supported_extensions.contains(&ext)
        } else {
            false
        }
    }

    fn process_single_file(&self, input_path: &PathBuf) -> Result<u64> {
        let output_path = self.config.get_output_path(input_path);
        
        if !self.config.quiet {
            println!("\n📷 Processing: {}", input_path.display());
        }
        
        let avif_size = self.converter.convert(input_path, &output_path)?;
        
        if self.config.delete_original && avif_size > 0 {
            fs::remove_file(input_path)
                .map_err(ConversionError::Io)?;
            if self.config.verbose {
                println!("   Deleted original: {}", input_path.display());
            }
        }
        
        Ok(avif_size)
    }

    fn collect_files(&self, root_dir: &PathBuf) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        let walker = if self.config.is_recursive() {
            WalkDir::new(root_dir)
                .follow_links(true)
                .into_iter()
        } else {
            WalkDir::new(root_dir)
                .max_depth(1)
                .into_iter()
        };
        
        for entry in walker.filter_entry(|e| {
            !e.file_name()
                .to_string_lossy()
                .starts_with('.')
        }) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && self.is_image_file(path) {
                    files.push(path.to_path_buf());
                }
            }
        }
        
        files.sort();
        files
    }

    pub fn run(&self) -> Result<()> {
        let input_path = self.config.get_input_path()
            .map_err(|e| ConversionError::InvalidParameter(e))?;
        
        if !self.config.is_directory_mode() {
            self.process_single_file(&input_path)?;
            return Ok(());
        }

        let files = self.collect_files(&input_path);
        
        if files.is_empty() {
            return Err(ConversionError::NoImagesFound(
                input_path.display().to_string()
            ));
        }

        if !self.config.quiet {
            println!("\n📁 Found {} image(s) in {}", files.len(), input_path.display());
            if self.config.is_recursive() {
                println!("   Recursive mode: enabled");
            }
        }

        let pb = if !self.config.quiet {
            let bar = ProgressBar::new(files.len() as u64);
            bar.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"));
            Some(bar)
        } else {
            None
        };

        let mut successful = 0;
        let mut failed = 0;
        let mut total_size = 0u64;

        for input_path in files {
            match self.process_single_file(&input_path) {
                Ok(size) => {
                    successful += 1;
                    total_size += size;
                },
                Err(e) => {
                    if !self.config.quiet {
                        println!("   ❌ Error processing {}: {}", input_path.display(), e);
                    }
                    failed += 1;
                }
            }
            
            if let Some(ref bar) = pb {
                bar.inc(1);
            }
        }

        if let Some(ref bar) = pb {
            bar.finish_with_message("done");
        }

        if !self.config.quiet {
            println!("\n📊 Summary:");
            println!("   Successful: {}", successful);
            if failed > 0 {
                println!("   Failed: {}", failed);
            }
            println!("   Total AVIF size: {:.2} KB", total_size as f64 / 1024.0);
        }

        Ok(())
    }
}

