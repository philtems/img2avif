use clap::Parser;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ColorSpaceArg {
    /// YUV 4:2:0 (standard, good quality/size tradeoff)
    Yuv420,
    /// YUV 4:4:4 (better color quality)
    Yuv444,
    /// RGB (maximum quality, larger files)
    Rgb,
}

impl fmt::Display for ColorSpaceArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpaceArg::Yuv420 => write!(f, "yuv420"),
            ColorSpaceArg::Yuv444 => write!(f, "yuv444"),
            ColorSpaceArg::Rgb => write!(f, "rgb"),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum BitDepthArg {
    /// 8 bits per channel (standard)
    Bit8,
    /// 10 bits per channel (better for gradients)
    Bit10,
    /// 12 bits per channel (professional quality)
    Bit12,
    /// Automatic based on source image
    Auto,
}

impl fmt::Display for BitDepthArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitDepthArg::Bit8 => write!(f, "8"),
            BitDepthArg::Bit10 => write!(f, "10"),
            BitDepthArg::Bit12 => write!(f, "12"),
            BitDepthArg::Auto => write!(f, "auto"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Philippe TEMESI <philippe@tems.be>",
    version = "1.0.0",
    about = "Convert images to AVIF format",
    long_about = "Convert images (JPEG, PNG, BMP, GIF, TIFF, WebP, ICO) to AVIF format with advanced quality settings.\n\
                  Supports single files, directories, and recursive processing.\n\
                  Website: https://www.tems.be - (c) 2026 Philippe TEMESI"
)]
pub struct Config {
    /// Input file (single file mode)
    #[arg(short = 'i', long = "input", value_name = "FILE", required_unless_present_any = ["directory", "recursive_dir"])]
    pub input: Option<PathBuf>,

    /// Input directory (process all images in this directory)
    #[arg(short = 'd', long = "directory", value_name = "DIR", conflicts_with = "input")]
    pub directory: Option<PathBuf>,

    /// Input directory with recursive processing
    #[arg(short = 'r', long = "recursive", value_name = "DIR", conflicts_with = "input")]
    pub recursive_dir: Option<PathBuf>,

    /// Output file or directory (default: input.avif or input_dir/*.avif)
    #[arg(short = 'o', long = "output", value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Delete original files after successful conversion
    #[arg(long = "delete", default_value_t = false)]
    pub delete_original: bool,

    /// Compression quality (0-100, default: 80)
    #[arg(short = 'q', long = "quality", default_value_t = 80.0)]
    pub quality: f32,

    /// Alpha channel quality (0-100, default: 80)
    #[arg(long = "alpha-quality", default_value_t = 80.0)]
    pub alpha_quality: f32,

    /// Encoding speed (0-10, default: 4, 0=best compression)
    #[arg(short = 's', long = "speed", default_value_t = 4)]
    pub speed: u8,

    /// Color space
    #[arg(long = "color-space", default_value_t = ColorSpaceArg::Yuv420)]
    pub color_space: ColorSpaceArg,

    /// Bit depth
    #[arg(long = "bit-depth", default_value_t = BitDepthArg::Auto)]
    pub bit_depth: BitDepthArg,

    /// Lossless mode
    #[arg(long = "lossless", default_value_t = false)]
    pub lossless: bool,

    /// Don't convert if AVIF file is larger than original
    #[arg(long = "discard-if-larger", default_value_t = false)]
    pub discard_if_larger: bool,

    /// Keep EXIF metadata (if available)
    #[arg(long = "keep-metadata", default_value_t = false)]
    pub keep_metadata: bool,

    /// Quiet mode (no messages)
    #[arg(short = 'q', long = "quiet", default_value_t = false, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    pub verbose: bool,
}

impl Config {
    pub fn get_input_path(&self) -> Result<PathBuf, String> {
        if let Some(path) = &self.input {
            Ok(path.clone())
        } else if let Some(path) = &self.directory {
            Ok(path.clone())
        } else if let Some(path) = &self.recursive_dir {
            Ok(path.clone())
        } else {
            Err("No input specified. Use -i for single file, -d for directory, or -r for recursive directory".to_string())
        }
    }

    pub fn is_directory_mode(&self) -> bool {
        self.directory.is_some() || self.recursive_dir.is_some()
    }

    pub fn is_recursive(&self) -> bool {
        self.recursive_dir.is_some()
    }

    pub fn get_output_path(&self, input_path: &PathBuf) -> PathBuf {
        if let Some(output) = &self.output {
            if self.is_directory_mode() && output.is_dir() {
                let input_stem = input_path.file_stem().unwrap_or_default();
                output.join(format!("{}.avif", input_stem.to_string_lossy()))
            } else {
                output.clone()
            }
        } else {
            let input_stem = input_path.file_stem().unwrap_or_default();
            let parent = input_path.parent().unwrap_or_else(|| std::path::Path::new("."));
            parent.join(format!("{}.avif", input_stem.to_string_lossy()))
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // First check if we have any input at all
        if self.input.is_none() && self.directory.is_none() && self.recursive_dir.is_none() {
            return Err("No input specified. Use -i for single file, -d for directory, or -r for recursive directory".to_string());
        }

        // Get the input path to validate
        let input_path = self.get_input_path()?;
        
        if !input_path.exists() {
            return Err(format!("Input path does not exist: {}", input_path.display()));
        }

        if !(0.0..=100.0).contains(&self.quality) {
            return Err("Quality must be between 0 and 100".to_string());
        }

        if !(0.0..=100.0).contains(&self.alpha_quality) {
            return Err("Alpha quality must be between 0 and 100".to_string());
        }

        if self.speed > 10 {
            return Err("Speed must be between 0 and 10".to_string());
        }

        // Validate input type matches mode
        if self.is_directory_mode() && !input_path.is_dir() {
            return Err("Directory mode requires a directory as input".to_string());
        }

        if !self.is_directory_mode() && self.input.is_some() && input_path.is_dir() {
            return Err("Input is a directory. Use -d for directory or -r for recursive mode".to_string());
        }

        Ok(())
    }
}

