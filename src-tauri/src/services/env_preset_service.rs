use std::path::PathBuf;
use crate::utils::env::load_env_presets_in_dir;
use crate::models::varo_node::EnvPreset;
use serde_json::Value;

pub fn load_env_presets_from_config(config: &Value) -> Vec<EnvPreset> {
    let dirs = config.pointer("/envPresets/directories")
        .and_then(|v| v.as_array())
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str())
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    let mut all_presets = Vec::new();

    for dir in dirs {
        match load_env_presets_in_dir(dir.to_str().unwrap_or_default()) {
            Ok(presets) => all_presets.extend(presets),
            Err(err) => {
                // You can optionally log the error here
                eprintln!("Failed to load presets from {:?}: {}", dir, err);
            }
        }
    }

    all_presets
}