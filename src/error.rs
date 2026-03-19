use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image decode error: {0}")]
    ImageDecode(String),

    #[error("AVIF encode error: {0}")]
    AvifEncode(String),

    #[error("Input file not found: {0}")]
    InputNotFound(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Raw image decode error: {0}")]
    RawDecode(String),

    #[error("No valid images found in directory: {0}")]
    NoImagesFound(String),
}

pub type Result<T> = std::result::Result<T, ConversionError>;

