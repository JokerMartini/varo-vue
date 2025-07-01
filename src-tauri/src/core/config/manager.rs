use serde_json::Value;
use crate::utils::config;

#[derive(Debug, Clone)]
pub struct ConfigManager {
    config: Value,
}

impl ConfigManager {
    pub fn load() -> Self {
        Self {
            config: config::load_config(),
        }
    }

    pub fn get_raw_config(&self) -> Value {
        self.config.clone()
    }

    pub fn reload(&mut self) {
        self.config = config::load_config();
    }
}