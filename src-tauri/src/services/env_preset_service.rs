use std::path::PathBuf;
use crate::utils::env::load_env_presets_in_dir;
use crate::models::varo_node::EnvPreset;

pub fn get_env_presets_for_frontend() -> Result<Vec<EnvPreset>, String> {
    let dir = PathBuf::from("C:/Users/jmartini/Documents/GitHub/varo-vue/test-data/envs");

    // Propagate the error if loading fails
    load_env_presets_in_dir(
        dir.to_str().ok_or_else(|| "Invalid preset directory path".to_string())?
    )
}
