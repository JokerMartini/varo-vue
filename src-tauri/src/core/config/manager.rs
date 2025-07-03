use serde_json::{Value, json};
use crate::utils::config;
use crate::utils::env::resolve_env_vars_recursive;
use crate::models::errors::{VaroError, VaroResult};

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: Value,
}

impl ConfigManager {
    /// Load and initialize the configuration with variable resolution
    pub fn load() -> VaroResult<Self> {
        let mut config = Self::load_merged_config();
        
        // Resolve environment variables recursively throughout the config
        resolve_env_vars_recursive(&mut config);
        
        Ok(Self { config })
    }

    /// Create an empty configuration manager
    pub fn empty() -> Self {
        Self {
            config: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    /// Get the fully resolved configuration
    pub fn get_config(&self) -> Value {
        self.config.clone()
    }

    /// Get a specific section of the config
    pub fn get_section(&self, section: &str) -> Value {
        self.config.get(section)
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}))
    }

    /// Reload the configuration from disk with variable resolution
    pub fn reload(&mut self) -> VaroResult<()> {
        let mut config = Self::load_merged_config();
        resolve_env_vars_recursive(&mut config);
        self.config = config;
        Ok(())
    }

    /// Get the default configuration structure
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

    /// Load and merge configuration from all sources
    fn load_merged_config() -> Value {
        let mut config = Self::default_config();
        
        // Load and merge environment-specified config
        if let Some(env_config) = config::load_env_config() {
            config::merge_configs(&mut config, &env_config);
        }
        
        // Load and merge user config
        if let Some(user_config) = config::load_user_config() {
            config::merge_configs(&mut config, &user_config);
        }
        
        // Debug output
        match serde_json::to_string_pretty(&config) {
            Ok(pretty_json) => println!("[Config Manager] Loaded config:\n{}", pretty_json),
            Err(err) => eprintln!("[Config Manager] Failed to serialize config: {}", err),
        }
        
        config
    }
}