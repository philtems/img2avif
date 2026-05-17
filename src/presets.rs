use std::path::Path;
use crate::config::{PresetArg, RawParams};
use crate::raw_decoder::RawDecoder;

/// Détection automatique du preset basé sur les métadonnées EXIF
pub fn detect_auto_preset(path: &Path) -> PresetArg {
    // Pour l'instant, version simple basée sur rawloader
    if !RawDecoder::is_raw_file(path) {
        return PresetArg::Landscape;
    }
    
    // Lire les métadonnées du RAW
    match rawloader::decode_file(path) {
        Ok(raw) => {
            // Détection basée sur ISO
            // Les valeurs ISO sont stockées dans raw.iso_speed
            let iso = raw.iso_speed;
            
            // Détection basée sur le type de scène (si disponible)
            // Pour l'instant, logique simple
            if iso > 1600 {
                // Haute ISO -> probablement nuit ou intérieur sombre
                PresetArg::Night
            } else if iso > 400 {
                // ISO moyen -> sport ou action
                PresetArg::Sports
            } else if iso <= 100 {
                // Bas ISO -> paysage ou extérieur
                PresetArg::Landscape
            } else {
                // Par défaut
                PresetArg::Natural
            }
        }
        Err(_) => PresetArg::Landscape,
    }
}

/// Retourne les paramètres pour un preset, avec détection auto si nécessaire
pub fn get_preset_params(preset: PresetArg, path: Option<&Path>) -> RawParams {
    match preset {
        PresetArg::Auto => {
            if let Some(p) = path {
                let detected = detect_auto_preset(p);
                RawParams::from_preset(detected)
            } else {
                RawParams::default()
            }
        }
        _ => RawParams::from_preset(preset),
    }
}

