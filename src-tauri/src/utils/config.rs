use std::fs;
use std::path::PathBuf;
use serde_json::{Value, json};
use crate::utils::env::expand_env_vars;


/// Get the path to the environment-specified config file
fn get_env_config_path() -> Option<PathBuf> {
    std::env::var("VARO_CONFIG_PATH").ok().map(PathBuf::from)
}

/// Get the path to the user-specific config file, creating directories as needed
fn get_user_config_path() -> Option<PathBuf> {
    if let Some(mut doc_path) = dirs::config_local_dir() {
        doc_path.push("Varo");
        
        // Ensure the Varo/ folder exists
        if std::fs::create_dir_all(&doc_path).is_err() {
            return None;
        }

        let config_path = doc_path.join("config.json");
        
        // Create empty config file if it doesn't exist
        if !config_path.exists() {
            let _ = std::fs::write(&config_path, "{}");
        }
        
        println!("[Config Utils] User config folder: {}", doc_path.display());
        Some(config_path)
    } else {
        None
    }
}

/// Load and parse a JSON config file from the given path
fn load_config_file(path: PathBuf) -> Value {
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_else(|| json!({}))
}

/// Recursively merge two JSON configurations, with overrides taking precedence
pub fn merge_configs(base: &mut Value, overrides: &Value) {
    match (base, overrides) {
        (Value::Object(base_map), Value::Object(override_map)) => {
            for (key, override_val) in override_map {
                merge_configs(base_map.entry(key).or_insert(Value::Null), override_val);
            }
        }
        (base, override_val) => {
            *base = override_val.clone();
        }
    }
}

/// Load configuration from environment-specified path
pub fn load_env_config() -> Option<Value> {
    get_env_config_path().map(load_config_file)
}

/// Load configuration from user-specific path
pub fn load_user_config() -> Option<Value> {
    get_user_config_path().map(load_config_file)
}
