use std::collections::HashMap;
use serde_json::Value;
use crate::models::entities::EnvPreset;
use crate::models::errors::{VaroError, VaroResult};
use crate::utils::env::load_env_presets_in_dir;

#[derive(Debug)]
pub struct PresetManager {
    presets: Vec<EnvPreset>,
    selected_preset: Option<EnvPreset>,
}

impl PresetManager {
    pub fn new(env_presets_config: &Value) -> VaroResult<Self> {
        let presets = Self::load_presets_from_env_config(env_presets_config)?;
        Ok(Self {
            presets,
            selected_preset: None,
        })
    }

    pub fn empty() -> Self {
        Self {
            presets: Vec::new(),
            selected_preset: None,
        }
    }

    pub fn get_all_presets(&self) -> Vec<EnvPreset> {
        self.presets.clone()
    }

    pub fn get_preset(&self, id: &str) -> Option<&EnvPreset> {
        self.presets.iter().find(|p| p.id == id)
    }

    pub fn select_preset(&mut self, id: &str) -> VaroResult<()> {
        if let Some(preset) = self.presets.iter().find(|p| p.id == id).cloned() {
            self.selected_preset = Some(preset);
            Ok(())
        } else {
            Err(VaroError::env_preset(format!("No EnvPreset found with id: {}", id)))
        }
    }

    pub fn get_selected_preset(&self) -> Option<&EnvPreset> {
        self.selected_preset.as_ref()
    }

    fn load_presets_from_env_config(env_presets_config: &Value) -> VaroResult<Vec<EnvPreset>> {
        // If config is empty or null, return empty list
        if env_presets_config.is_null() || env_presets_config.as_object().map_or(true, |o| o.is_empty()) {
            return Ok(Vec::new());
        }

        // Get resolved directories (these should already be validated and existing)
        let dirs = env_presets_config
            .get("directories")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(std::path::PathBuf::from)
            .collect::<Vec<_>>();
        
        if dirs.is_empty() {
            return Ok(Vec::new());
        }

        let mut all_presets = Vec::new();
        let mut errors = Vec::new();

        for dir in dirs {
            match load_env_presets_in_dir(dir.to_str().unwrap_or_default()) {
                Ok(presets) => all_presets.extend(presets),
                Err(err) => {
                    errors.push(format!("Failed to load presets from {:?}: {}", dir, err));
                }
            }
        }

        // Log warnings but don't fail completely
        for error in errors {
            eprintln!("Warning: {}", error);
        }

        Ok(all_presets)
    }

    pub fn reload(&mut self, env_presets_config: &Value) -> VaroResult<()> {
        self.presets = Self::load_presets_from_env_config(env_presets_config)?;
        
        // Clear selected preset if it no longer exists
        if let Some(ref selected) = self.selected_preset {
            if !self.presets.iter().any(|p| p.id == selected.id) {
                self.selected_preset = None;
            }
        }
        
        Ok(())
    }
}