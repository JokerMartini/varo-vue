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

    /// Returns the resolved config
    pub fn get_config(&self) -> Value {
        self.config.clone()
    }

    /// Get a specific section of the config
    pub fn get_section(&self, section: &str) -> Value {
        self.config.get(section)
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}))
    }

    pub fn reload(&mut self) -> VaroResult<()> {
        self.config = config::load_config();
        Ok(())
    }
}