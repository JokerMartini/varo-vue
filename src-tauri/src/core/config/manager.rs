use serde_json::Value;
use crate::utils::config;
use crate::models::errors::{VaroError, VaroResult};

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: Value,
}

impl ConfigManager {
    pub fn load() -> VaroResult<Self> {
        let config = config::load_config();
        Ok(Self { config })
    }

    pub fn empty() -> Self {
        Self {
            config: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    pub fn get_raw_config(&self) -> Value {
        self.config.clone()
    }

    pub fn reload(&mut self) -> VaroResult<()> {
        self.config = config::load_config();
        Ok(())
    }

    pub fn get_env_preset_directories(&self) -> Vec<String> {
        self.config.pointer("/env_presets/directories")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect()
    }
}