use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;

use crate::models::entities::{EnvPreset, VaroNode};
use crate::models::errors::{VaroError, VaroResult};
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
        
        // Load configuration file
        let config_manager = match ConfigManager::load() {
            Ok(manager) => manager,
            Err(e) => {
                eprintln!("Warning: Failed to load config: {}", e);
                ConfigManager::empty()
            }
        };
        
        // Load env presets based on config
        let env_presets_config = config_manager.get_section("env_presets");
        let preset_manager = match PresetManager::new(&env_presets_config) {
            Ok(manager) => manager,
            Err(e) => {
                eprintln!("Warning: Failed to load presets: {}", e);
                PresetManager::empty()
            }
        };
        
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
    pub async fn refresh_env_preset(&self, preset_id: &str) -> VaroResult<()> {
        let mut preset_manager = self.preset_manager.write().await;
        preset_manager.select_preset(preset_id)?;
        
        if let Some(preset) = preset_manager.get_selected_preset() {
            let mut node_manager = self.node_manager.write().await;
            node_manager.refresh_with_preset(preset)?;
        }
        
        Ok(())
    }

    pub async fn execute_node(&self, node_id: &str) -> VaroResult<()> {
        let node_manager = self.node_manager.read().await;
        let preset_manager = self.preset_manager.read().await;
        let selected_preset = preset_manager.get_selected_preset();
        
        node_manager.execute_node_with_env_expansion(node_id, selected_preset)
    }

    pub async fn get_all_presets(&self) -> Vec<EnvPreset> {
        self.preset_manager.read().await.get_all_presets()
    }

    pub async fn get_config(&self) -> Value {
        self.config_manager.read().await.get_config()
    }

    // Sync methods for Tauri compatibility (using blocking operations)
    pub fn sync_get_all_presets(&self) -> Vec<EnvPreset> {
        self.preset_manager.blocking_read().get_all_presets()
    }

    pub fn sync_get_selected_preset(&self) -> Option<EnvPreset> {
        self.preset_manager.blocking_read().get_selected_preset().cloned()
    }

    pub fn sync_select_preset(&self, preset_id: &str) -> VaroResult<()> {
        println!("[VaroCore] Selecting preset: {}", preset_id);
        
        // Select the preset
        let mut preset_manager = self.preset_manager.blocking_write();
        preset_manager.select_preset(preset_id)?;
        
        // Get the selected preset for node refresh
        let selected_preset = preset_manager.get_selected_preset().cloned();
        drop(preset_manager); // Release preset lock
        
        // Reload nodes with the new preset (which may have changed VARO_PATH)
        let mut node_manager = self.node_manager.blocking_write();
        if let Some(preset) = selected_preset {
            node_manager.refresh_with_preset(&preset)?;
        } else {
            node_manager.load_nodes_from_varo_path()?;
        }
        
        println!("[VaroCore] Preset selection and node refresh complete");
        Ok(())
    }

    pub fn sync_get_config(&self) -> Value {
        self.config_manager.blocking_read().get_config()
    }

    pub fn sync_reload_config(&self) -> VaroResult<()> {
        println!("[VaroCore] Starting config reload...");
        
        let mut config_manager = self.config_manager.blocking_write();
        config_manager.reload()?;
        println!("[VaroCore] Config reloaded successfully");
        
        // Also reload presets with new config
        let env_presets_config = config_manager.get_section("env_presets");
        drop(config_manager); // Release config lock
        
        let mut preset_manager = self.preset_manager.blocking_write();
        preset_manager.reload(&env_presets_config)?;
        println!("[VaroCore] Presets reloaded successfully");
        drop(preset_manager); // Release preset lock
        
        // Also reload nodes from disk
        let mut node_manager = self.node_manager.blocking_write();
        node_manager.load_nodes_from_varo_path()?;
        println!("[VaroCore] Nodes reloaded successfully");
        
        println!("[VaroCore] Complete config reload finished successfully");
        Ok(())
    }

    pub fn get_username(&self) -> &str {
        self.system_info.get_username()
    }

    pub fn get_platform(&self) -> &str {
        self.system_info.get_platform()
    }

    pub fn sync_get_all_nodes(&self) -> Vec<crate::models::entities::VaroNode> {
        self.node_manager.blocking_read().get_all_nodes()
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn sync_execute_node(&self, node_id: &str) -> VaroResult<()> {
        let node_manager = self.node_manager.clone();
        let preset_manager = self.preset_manager.clone();
        let node_id = node_id.to_string();
        
        // Execute in a separate thread to avoid blocking the UI
        std::thread::spawn(move || {
            let node_manager = node_manager.blocking_read();
            let preset_manager = preset_manager.blocking_read();
            let selected_preset = preset_manager.get_selected_preset();
            
            if let Err(e) = node_manager.execute_node_with_env_expansion(&node_id, selected_preset) {
                eprintln!("[Node Execution] Error executing node {}: {}", node_id, e);
            }
        });
        
        Ok(())
    }

    pub fn sync_show_node_in_folder(&self, node_id: &str) -> VaroResult<()> {
        self.node_manager.blocking_read().show_node_in_folder(node_id)
    }
}