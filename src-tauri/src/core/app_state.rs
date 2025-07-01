use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::entities::{EnvPreset, VaroNode};
use crate::core::config::ConfigManager;
use crate::core::env_presets::PresetManager;
use crate::core::nodes::NodeManager;
use crate::core::system::SystemInfo;

#[derive(Debug)]
pub struct VaroCore {
    pub config_manager: Arc<RwLock<ConfigManager>>,
    pub preset_manager: Arc<RwLock<PresetManager>>,
    pub node_manager: Arc<RwLock<NodeManager>>,
    pub system_info: SystemInfo,
}

impl VaroCore {
    pub fn new() -> Self {
        let system_info = SystemInfo::collect();
        let config_manager = ConfigManager::load();
        let preset_manager = PresetManager::new(&config_manager.get_raw_config());
        let mut node_manager = NodeManager::new();
        
        // Try to load nodes initially
        if let Err(e) = node_manager.load_nodes_from_varo_path() {
            eprintln!("Warning: Failed to load nodes: {}", e);
        }

        Self {
            config_manager: Arc::new(RwLock::new(config_manager)),
            preset_manager: Arc::new(RwLock::new(preset_manager)),
            node_manager: Arc::new(RwLock::new(node_manager)),
            system_info,
        }
    }

    // Async methods for CLI/advanced usage
    pub async fn refresh_env_preset(&self, preset_id: &str) -> Result<(), String> {
        let mut preset_manager = self.preset_manager.write().await;
        preset_manager.select_preset(preset_id)?;
        
        if let Some(preset) = preset_manager.get_selected_preset() {
            let mut node_manager = self.node_manager.write().await;
            node_manager.refresh_with_preset(preset)?;
        }
        
        Ok(())
    }

    pub async fn execute_node(&self, node_id: &str) -> Result<(), String> {
        let node_manager = self.node_manager.read().await;
        node_manager.execute_node(node_id)
    }

    pub async fn get_all_presets(&self) -> Vec<EnvPreset> {
        self.preset_manager.read().await.get_all_presets()
    }

    pub async fn get_config(&self) -> Value {
        self.config_manager.read().await.get_raw_config()
    }

    // Sync methods for Tauri compatibility (using blocking operations)
    pub fn sync_get_all_presets(&self) -> Vec<EnvPreset> {
        // Use blocking_read in sync context since Tauri commands are sync
        self.preset_manager.blocking_read().get_all_presets()
    }

    pub fn sync_select_preset(&self, preset_id: &str) -> Result<(), String> {
        self.preset_manager.blocking_write().select_preset(preset_id)
    }

    pub fn sync_get_config(&self) -> Value {
        self.config_manager.blocking_read().get_raw_config()
    }

    pub fn sync_reload_config(&self) -> Result<(), String> {
        self.config_manager.blocking_write().reload();
        Ok(())
    }

    pub fn get_username(&self) -> &str {
        self.system_info.get_username()
    }

    pub fn get_platform(&self) -> &str {
        self.system_info.get_platform()
    }
}