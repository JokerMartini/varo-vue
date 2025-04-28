use blake3;
use std::path::{Path};

pub struct Hasher;

impl Hasher {
    /// Normalize the path by extracting the filename and lowercasing it
    pub fn normalize(path: &Path) -> String {
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default()
    }

    /// Generate a stable ID from any input string
    pub fn generate_id_from_str(input: &str) -> String {
        let hash = blake3::hash(input.as_bytes());
        let hex_string = hash.to_hex();
        hex_string[..12.min(hex_string.len())].to_string()
    }

    /// Generate an ID directly from a Path (shortcut method)
    pub fn generate_id_from_path(path: &Path) -> String {
        let normalized = Self::normalize(path);
        Self::generate_id_from_str(&normalized)
    }
}