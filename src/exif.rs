use std::path::Path;
use crate::error::Result;
use image::DynamicImage;
use crate::raw_decoder::RawDecoder;

/// Récupère l'orientation d'un fichier (priorité RAW, puis fallback)
pub fn get_orientation(path: &Path) -> Result<u16> {
    // Pour les fichiers RAW, utiliser rawloader
    if RawDecoder::is_raw_file(path) {
        return RawDecoder::get_raw_orientation(path);
    }
    
    // Pour les autres formats, orientation par défaut
    Ok(1)
}

/// Applique une rotation à une image DynamicImage selon l'orientation
pub fn apply_rotation(img: &DynamicImage, orientation: u16) -> DynamicImage {
    match orientation {
        1 => img.clone(),
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate90().flipv(),
        8 => img.rotate270(),
        _ => img.clone(),
    }
}

/// Applique une rotation manuelle
pub fn apply_manual_rotation(img: &DynamicImage, rotate: crate::config::RotateArg) -> DynamicImage {
    use crate::config::RotateArg;
    match rotate {
        RotateArg::None => img.clone(),
        RotateArg::Rotate90 => img.rotate90(),
        RotateArg::Rotate180 => img.rotate180(),
        RotateArg::Rotate270 => img.rotate270(),
        RotateArg::Auto => img.clone(),
    }
}

