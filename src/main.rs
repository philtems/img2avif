use clap::Parser;
use std::process;

mod config;
mod converter;
mod processor;
mod error;
mod raw_decoder;
mod exif;

use config::Config;
use processor::ImageProcessor;

fn main() {
    let config = Config::parse();

    if let Err(e) = config.validate() {
        eprintln!("❌ Configuration error: {}", e);
        process::exit(1);
    }

    if !config.quiet {
        println!("🖼️  img2avif v{}", env!("CARGO_PKG_VERSION"));
        println!("   © 2026 Philippe TEMESI - https://www.tems.be");
        println!("   Supported formats: JPEG, PNG, BMP, GIF, TIFF, WebP, ICO, RAW (NEF, CR2, ARW, DNG, etc.)");
        println!();
    }

    let processor = ImageProcessor::new(config);

    if let Err(e) = processor.run() {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

