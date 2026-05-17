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

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum PresetArg {
    /// Landscape / outdoor (default)
    Landscape,
    /// Portrait (natural skin tones)
    Portrait,
    /// Vivid (high saturation)
    Vivid,
    /// Natural (minimal processing)
    Natural,
    /// Flat (maximum latitude for editing)
    Flat,
    /// Night (boost exposure for low light)
    Night,
    /// Black & white conversion
    BlackWhite,
    /// Architecture (buildings, lines)
    Architecture,
    /// Macro (close-up, flowers)
    Macro,
    /// Sports (action shots)
    Sports,
    /// Sunset (warm tones)
    Sunset,
    /// Winter (snow, bright scenes)
    Winter,
    /// Forest (vegetation, green)
    Forest,
    /// Street (urban photography)
    Street,
    /// Auto (automatic detection based on EXIF)
    Auto,
}

impl fmt::Display for PresetArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PresetArg::Landscape => write!(f, "landscape"),
            PresetArg::Portrait => write!(f, "portrait"),
            PresetArg::Vivid => write!(f, "vivid"),
            PresetArg::Natural => write!(f, "natural"),
            PresetArg::Flat => write!(f, "flat"),
            PresetArg::Night => write!(f, "night"),
            PresetArg::BlackWhite => write!(f, "black-white"),
            PresetArg::Architecture => write!(f, "architecture"),
            PresetArg::Macro => write!(f, "macro"),
            PresetArg::Sports => write!(f, "sports"),
            PresetArg::Sunset => write!(f, "sunset"),
            PresetArg::Winter => write!(f, "winter"),
            PresetArg::Forest => write!(f, "forest"),
            PresetArg::Street => write!(f, "street"),
            PresetArg::Auto => write!(f, "auto"),
        }
    }
}

/// Paramètres RAW bruts (sans preset)
#[derive(Debug, Clone, Copy)]
pub struct RawParams {
    pub exposure: f64,
    pub saturation: f64,
    pub contrast: f64,
    pub gamma: f64,
    pub highlight: f64,
    pub temperature: f64,
}

impl Default for RawParams {
    fn default() -> Self {
        // Default = preset Landscape
        Self {
            exposure: 1.60,
            saturation: 1.35,
            contrast: 1.20,
            gamma: 1.95,
            highlight: 1.00,
            temperature: 0.0,
        }
    }
}

impl RawParams {
    /// Retourne les paramètres pour un preset donné
    pub fn from_preset(preset: PresetArg) -> Self {
        match preset {
            PresetArg::Landscape => Self {
                exposure: 1.60,
                saturation: 1.35,
                contrast: 1.20,
                gamma: 1.95,
                highlight: 1.00,
                temperature: 0.0,
            },
            PresetArg::Portrait => Self {
                exposure: 1.20,
                saturation: 1.05,
                contrast: 1.02,
                gamma: 2.20,
                highlight: 0.90,
                temperature: 0.0,
            },
            PresetArg::Vivid => Self {
                exposure: 1.40,
                saturation: 1.45,
                contrast: 1.15,
                gamma: 2.00,
                highlight: 0.95,
                temperature: 0.0,
            },
            PresetArg::Natural => Self {
                exposure: 1.30,
                saturation: 1.10,
                contrast: 1.05,
                gamma: 2.20,
                highlight: 0.85,
                temperature: 0.0,
            },
            PresetArg::Flat => Self {
                exposure: 1.20,
                saturation: 0.90,
                contrast: 0.95,
                gamma: 2.40,
                highlight: 0.80,
                temperature: 0.0,
            },
            PresetArg::Night => Self {
                exposure: 1.90,
                saturation: 1.20,
                contrast: 1.25,
                gamma: 2.00,
                highlight: 0.70,
                temperature: 0.0,
            },
            PresetArg::BlackWhite => Self {
                exposure: 1.40,
                saturation: 0.00,
                contrast: 1.30,
                gamma: 2.00,
                highlight: 0.90,
                temperature: 0.0,
            },
            PresetArg::Architecture => Self {
                exposure: 1.40,
                saturation: 1.20,
                contrast: 1.25,
                gamma: 2.10,
                highlight: 0.95,
                temperature: 0.0,
            },
            PresetArg::Macro => Self {
                exposure: 1.50,
                saturation: 1.30,
                contrast: 1.15,
                gamma: 2.00,
                highlight: 0.90,
                temperature: 0.0,
            },
            PresetArg::Sports => Self {
                exposure: 1.45,
                saturation: 1.35,
                contrast: 1.20,
                gamma: 2.00,
                highlight: 0.95,
                temperature: 0.0,
            },
            PresetArg::Sunset => Self {
                exposure: 1.70,
                saturation: 1.40,
                contrast: 1.15,
                gamma: 1.95,
                highlight: 0.98,
                temperature: 4500.0, // Coucher de soleil - plus chaud
            },
            PresetArg::Winter => Self {
                exposure: 1.55,
                saturation: 1.15,
                contrast: 1.18,
                gamma: 2.10,
                highlight: 0.92,
                temperature: 0.0,
            },
            PresetArg::Forest => Self {
                exposure: 1.50,
                saturation: 1.38,
                contrast: 1.22,
                gamma: 1.98,
                highlight: 0.96,
                temperature: 0.0,
            },
            PresetArg::Street => Self {
                exposure: 1.35,
                saturation: 1.25,
                contrast: 1.18,
                gamma: 2.05,
                highlight: 0.88,
                temperature: 0.0,
            },
            PresetArg::Auto => Self::default(), // Sera remplacé par détection
        }
    }

    /// Affiche les paramètres sous forme de chaîne
    pub fn to_string(&self) -> String {
        format!(
            "exposure={:.2}, saturation={:.2}, contrast={:.2}, gamma={:.2}, highlight={:.2}, temperature={:.0}",
            self.exposure, self.saturation, self.contrast, self.gamma, self.highlight, self.temperature
        )
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Philippe TEMESI <philippe@tems.be>",
    version = "1.0.0",
    about = "Convert images to AVIF format",
    long_about = "Convert images (JPEG, PNG, BMP, GIF, TIFF, WebP, ICO, RAW) to AVIF format with advanced quality settings.\n\
                  Supports single files, directories, and recursive processing.\n\
                  RAW development parameters allow fine-tuning of exposure, saturation, and contrast.\n\
                  Use --preset for predefined settings (default: landscape).\n\
                  Individual raw-* parameters override preset values.\n\
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

    // ========== PARAMÈTRES DE DÉVELOPPEMENT RAW ==========
    
    /// Preset for RAW development (overridden by individual raw-* parameters)
    #[arg(long = "preset", value_enum, default_value_t = PresetArg::Landscape)]
    pub preset: PresetArg,

    /// Exposure compensation for RAW files (0.5-2.0)
    #[arg(long = "raw-exposure")]
    pub raw_exposure: Option<f64>,

    /// Saturation boost for RAW files (0.0-2.0)
    #[arg(long = "raw-saturation")]
    pub raw_saturation: Option<f64>,

    /// Contrast boost for RAW files (0.0-2.0)
    #[arg(long = "raw-contrast")]
    pub raw_contrast: Option<f64>,

    /// White balance temperature for RAW files (0=auto, 2000-12000K)
    #[arg(long = "raw-temperature")]
    pub raw_temperature: Option<f64>,

    /// Gamma correction for RAW files (1.8-2.6)
    #[arg(long = "raw-gamma")]
    pub raw_gamma: Option<f64>,

    /// Highlight target (0.5-1.0)
    #[arg(long = "raw-highlight")]
    pub raw_highlight_target: Option<f64>,

    // ========== PARAMÈTRES DE ROTATION ==========
    
    /// Rotate image (auto, none, 90, 180, 270) - default: auto
    #[arg(long = "rotate", default_value_t = RotateArg::Auto)]
    pub rotate: RotateArg,
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

    /// Obtient les paramètres RAW finaux (preset + overrides individuels)
    pub fn get_raw_params(&self) -> RawParams {
        let preset_params = if matches!(self.preset, PresetArg::Auto) {
            // Pour auto, on utilisera la détection plus tard
            // Temporairement, on retourne landscape
            RawParams::from_preset(PresetArg::Landscape)
        } else {
            RawParams::from_preset(self.preset)
        };
        
        RawParams {
            exposure: self.raw_exposure.unwrap_or(preset_params.exposure),
            saturation: self.raw_saturation.unwrap_or(preset_params.saturation),
            contrast: self.raw_contrast.unwrap_or(preset_params.contrast),
            gamma: self.raw_gamma.unwrap_or(preset_params.gamma),
            highlight: self.raw_highlight_target.unwrap_or(preset_params.highlight),
            temperature: self.raw_temperature.unwrap_or(preset_params.temperature),
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

        // Validate RAW parameters if provided
        if let Some(exposure) = self.raw_exposure {
            if !(0.5..=2.0).contains(&exposure) {
                return Err("RAW exposure must be between 0.5 and 2.0".to_string());
            }
        }

        if let Some(saturation) = self.raw_saturation {
            if !(0.0..=2.0).contains(&saturation) {
                return Err("RAW saturation must be between 0.0 and 2.0".to_string());
            }
        }

        if let Some(contrast) = self.raw_contrast {
            if !(0.0..=2.0).contains(&contrast) {
                return Err("RAW contrast must be between 0.0 and 2.0".to_string());
            }
        }

        if let Some(temperature) = self.raw_temperature {
            if temperature != 0.0 && !(2000.0..=12000.0).contains(&temperature) {
                return Err("RAW temperature must be 0 (auto) or between 2000 and 12000 Kelvin".to_string());
            }
        }

        if let Some(gamma) = self.raw_gamma {
            if !(1.8..=2.6).contains(&gamma) {
                return Err("RAW gamma must be between 1.8 and 2.6".to_string());
            }
        }

        if let Some(highlight) = self.raw_highlight_target {
            if !(0.5..=1.0).contains(&highlight) {
                return Err("RAW highlight target must be between 0.5 and 1.0".to_string());
            }
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

