use std::path::PathBuf;
use std::fs;
use base64::engine::general_purpose;
use base64::Engine;
use crate::utils::env::expand_env_vars;

/// Resolves the full path to an icon file by expanding environment variables
/// and prepending VARO_PATH if the input is a relative path.
/// Prints errors to stderr if anything goes wrong.
pub fn resolve_icon_file_path(raw_icon_path: &str) -> Option<PathBuf> {
    if raw_icon_path.trim().is_empty() {
        eprintln!("Icon path is empty — skipping.");
        return None;
    }

    let expanded_path = expand_env_vars(raw_icon_path);

    let varo_root = match std::env::var("VARO_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            eprintln!("Environment variable 'VARO_PATH' is not set — cannot resolve icon path.");
            return None;
        }
    };

    let full_icon_path = if PathBuf::from(&expanded_path).is_absolute() {
        PathBuf::from(&expanded_path)
    } else {
        varo_root.join("icons").join(&expanded_path)
    };

    if !full_icon_path.exists() {
        eprintln!("Icon file does not exist at path: {}", full_icon_path.display());
        return None;
    }

    Some(full_icon_path)
}

/// Loads the contents of an icon file and converts it into an inline data URI.
/// Returns an empty string on failure.
pub fn load_icon_data_uri(icon_path: &PathBuf) -> String {
    match icon_path.extension().and_then(|ext| ext.to_str()) {
        Some("svg") => fs::read_to_string(icon_path).unwrap_or_default(),
        Some(ext) => {
            let mime = format!("image/{}", ext);
            match fs::read(icon_path) {
                Ok(bytes) => format!(
                    "data:{};base64,{}",
                    mime,
                    general_purpose::STANDARD.encode(bytes)
                ),
                Err(_) => "".to_string(),
            }
        }
        None => "".to_string(),
    }
}