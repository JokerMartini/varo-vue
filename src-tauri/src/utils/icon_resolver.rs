use std::path::PathBuf;
use std::fs;
use base64::engine::general_purpose;
use base64::Engine;

pub fn resolve_icon_path(raw_icon_path: &str, warnings: &mut Vec<String>, node_id: &str) -> String {
    if raw_icon_path.is_empty() {
        warnings.push(format!("Node '{}' missing optional 'icon' field", node_id));
        return "".to_string();
    }

    let raw_icon_path = crate::utils::env::expand_env_vars(raw_icon_path);

    let varo_root = match std::env::var("VARO_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            warnings.push(format!("Node '{}': VARO_PATH not set", node_id));
            return "".to_string();
        }
    };

    let full_icon_path = if PathBuf::from(&raw_icon_path).is_absolute() {
        PathBuf::from(&raw_icon_path)
    } else {
        varo_root.join("icons").join(raw_icon_path)
    };

    if !full_icon_path.exists() {
        warnings.push(format!("Node '{}' icon file missing: {}", node_id, full_icon_path.display()));
        return "".to_string();
    }

    match full_icon_path.extension().and_then(|ext| ext.to_str()) {
        Some("svg") => fs::read_to_string(&full_icon_path)
            .unwrap_or_else(|_| "".to_string()),
        Some(ext) => {
            let mime = format!("image/{}", ext);
            match fs::read(&full_icon_path) {
                Ok(bytes) => format!("data:{};base64,{}", mime, general_purpose::STANDARD.encode(bytes)),
                Err(_) => "".to_string(),
            }
        }
        None => "".to_string(),
    }
}
