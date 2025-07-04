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
/// Handles SVG files specially by sanitizing them and creating proper data URIs.
/// Returns an empty string on failure.
pub fn load_icon_data_uri(icon_path: &PathBuf) -> String {
    match icon_path.extension().and_then(|ext| ext.to_str()) {
        Some("svg") => load_svg_as_data_uri(icon_path),
        Some(ext) => load_image_as_data_uri(icon_path, ext),
        None => {
            eprintln!("Icon file has no extension: {}", icon_path.display());
            String::new()
        }
    }
}

/// Loads an SVG file and converts it to a proper data URI with sanitization
fn load_svg_as_data_uri(icon_path: &PathBuf) -> String {
    match fs::read_to_string(icon_path) {
        Ok(svg_content) => {
            let sanitized_svg = sanitize_svg_content(&svg_content);
            if sanitized_svg.is_empty() {
                eprintln!("SVG file is empty or invalid: {}", icon_path.display());
                return String::new();
            }
            
            // Create proper data URI for SVG
            format!("data:image/svg+xml;base64,{}", 
                general_purpose::STANDARD.encode(sanitized_svg.as_bytes()))
        }
        Err(e) => {
            eprintln!("Failed to read SVG file {}: {}", icon_path.display(), e);
            String::new()
        }
    }
}

/// Loads a raster image file and converts it to a data URI
fn load_image_as_data_uri(icon_path: &PathBuf, extension: &str) -> String {
    let mime_type = get_mime_type_for_extension(extension);
    
    match fs::read(icon_path) {
        Ok(bytes) => {
            if bytes.is_empty() {
                eprintln!("Image file is empty: {}", icon_path.display());
                return String::new();
            }
            
            format!("data:{};base64,{}", mime_type, general_purpose::STANDARD.encode(bytes))
        }
        Err(e) => {
            eprintln!("Failed to read image file {}: {}", icon_path.display(), e);
            String::new()
        }
    }
}

/// Sanitizes SVG content by extracting just the SVG element and its contents
fn sanitize_svg_content(svg_content: &str) -> String {
    let trimmed = svg_content.trim();
    
    if trimmed.is_empty() {
        return String::new();
    }
    
    // Find the <svg> element start (case insensitive)
    let content_lower = trimmed.to_lowercase();
    if let Some(svg_start) = content_lower.find("<svg") {
        // Verify it's actually an SVG tag (followed by space, > or />)
        if let Some(char_after) = content_lower.chars().nth(svg_start + 4) {
            if char_after.is_whitespace() || char_after == '>' || char_after == '/' {
                // Return everything from <svg to the end (which includes the closing </svg>)
                return trimmed[svg_start..].to_string();
            }
        }
    }
    
    // If no valid <svg> tag found, return original but warn
    eprintln!("Warning: No valid <svg> element found in SVG file");
    trimmed.to_string()
}

/// Maps file extensions to proper MIME types
fn get_mime_type_for_extension(extension: &str) -> &'static str {
    match extension.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        "tiff" | "tif" => "image/tiff",
        "svg" => "image/svg+xml",
        "avif" => "image/avif",
        "heic" | "heif" => "image/heic",
        _ => "image/png", // Default fallback
    }
}

