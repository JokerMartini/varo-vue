use std::fs;
use std::path::PathBuf;
use serde_json::{Value, json};
use crate::utils::env::expand_env_vars;

fn default_config() -> Value {
    json!({
        "env_presets": { 
            "directories": [], 
            "default_id": null 
        },
        "ui": { 
            "dark_mode": true, 
            "show_groups": false, 
            "show_categories": true, 
            "show_hidden_nodes": false 
        }
    })
}

fn get_env_config_path() -> Option<PathBuf> {
    std::env::var("VARO_CONFIG_PATH").ok().map(PathBuf::from)
}

fn get_user_config_path() -> Option<PathBuf> {
    if let Some(mut doc_path) = dirs::config_local_dir() {
        doc_path.push("Varo");
        
        // Ensure the Varo/ folder exists
        if std::fs::create_dir_all(&doc_path).is_err() {
            return None;
        }

        let config_path = doc_path.join("config.json");
        
        if !config_path.exists() {
            let _ = std::fs::write(&config_path, "{}");
        }
        
        println!("[Varo Config] User config folder: {}", doc_path.display());
        Some(config_path)
    } else {
        None
    }
}

fn load_config_file(path: PathBuf) -> Value {
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_else(|| json!({}))
}

fn merge_configs(base: &mut Value, overrides: &Value) {
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

pub fn load_config() -> Value {
    let mut config = default_config();

    if let Some(global_path) = get_env_config_path() {
        merge_configs(&mut config, &load_config_file(global_path));
    }

    if let Some(user_path) = get_user_config_path() {
        merge_configs(&mut config, &load_config_file(user_path));
    }

    match serde_json::to_string_pretty(&config) {
        Ok(pretty_json) => println!("[Varo Config] Merged config:\n{}", pretty_json),
        Err(err) => eprintln!("[Varo Config] Failed to serialize config: {}", err),
    }

    config
}
