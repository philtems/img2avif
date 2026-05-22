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

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum RotateArg {
    /// Auto-rotate based on EXIF orientation
    Auto,
    /// No rotation
    None,
    /// Rotate 90 degrees clockwise
    Rotate90,
    /// Rotate 180 degrees
    Rotate180,
    /// Rotate 270 degrees clockwise (or 90 counter-clockwise)
    Rotate270,
}

impl fmt::Display for RotateArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RotateArg::Auto => write!(f, "auto"),
            RotateArg::None => write!(f, "none"),
            RotateArg::Rotate90 => write!(f, "90"),
            RotateArg::Rotate180 => write!(f, "180"),
            RotateArg::Rotate270 => write!(f, "270"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Philippe TEMESI <philippe@tems.be>",
    version = "1.3.0",
    about = "Convert images to AVIF format",
    long_about = "Convert images (JPEG, PNG, BMP, GIF, TIFF, WebP, ICO, RAW) to AVIF format with advanced quality settings.\n\
                  Supports single files, directories, and recursive processing.\n\
                  RAW files are processed with rawloader's high-quality demosaicing.\n\
                  Auto-exposure uses histogram analysis for optimal brightness.\n\
                  Auto-saturation compensates for color loss when exposure is increased.\n\
                  Rotation: --rotate auto (default) reads EXIF orientation, or manual (none/90/180/270).\n\
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

    // ========== PARAMÈTRES DE ROTATION ==========
    
    /// Rotate image (auto, none, 90, 180, 270) - default: auto
    #[arg(long = "rotate", default_value_t = RotateArg::Auto)]
    pub rotate: RotateArg,

    // ========== PARAMÈTRES RAW ==========
    
    /// RAW exposure compensation (auto, or 0.5-5.0)
    #[arg(long = "raw-exposure", default_value_t = String::from("auto"))]
    pub raw_exposure: String,

    /// Target percentile for auto-exposure (0.5-1.0, default: 0.95)
    #[arg(long = "raw-percentile", default_value_t = 0.95)]
    pub raw_percentile: f32,

    /// RAW saturation (auto, or 0.5-2.0, default: auto)
    #[arg(long = "raw-saturation", default_value_t = String::from("auto"))]
    pub raw_saturation: String,

    /// RAW contrast (auto, or 0.5-2.0, default: 1.0)
    #[arg(long = "raw-contrast", default_value_t = 1.0)]
    pub raw_contrast: f32,
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

        // Validate RAW parameters
        if self.raw_exposure != "auto" {
            if let Ok(val) = self.raw_exposure.parse::<f32>() {
                if !(0.5..=5.0).contains(&val) {
                    return Err("RAW exposure must be between 0.5 and 5.0".to_string());
                }
            } else {
                return Err("RAW exposure must be 'auto' or a number between 0.5 and 5.0".to_string());
            }
        }

        if !(0.5..=1.0).contains(&self.raw_percentile) {
            return Err("RAW percentile must be between 0.5 and 1.0".to_string());
        }

        if self.raw_saturation != "auto" {
            if let Ok(val) = self.raw_saturation.parse::<f32>() {
                if !(0.5..=2.0).contains(&val) {
                    return Err("RAW saturation must be between 0.5 and 2.0".to_string());
                }
            } else {
                return Err("RAW saturation must be 'auto' or a number between 0.5 and 2.0".to_string());
            }
        }

        if !(0.5..=2.0).contains(&self.raw_contrast) {
            return Err("RAW contrast must be between 0.5 and 2.0".to_string());
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

