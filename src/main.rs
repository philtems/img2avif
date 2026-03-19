use clap::Parser;
use std::process;

mod config;
mod converter;
mod processor;
mod error;

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
        println!("   Supported formats: JPEG, PNG, BMP, GIF, TIFF, WebP, ICO");
        println!();
    }

    let processor = ImageProcessor::new(config);

    if let Err(e) = processor.run() {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

