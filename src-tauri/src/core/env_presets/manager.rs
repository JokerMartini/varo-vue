use std::collections::HashMap;
use serde_json::Value;
use crate::models::entities::EnvPreset;
use crate::utils::env::{load_env_presets_in_dir, expand_env_vars};
use std::path::PathBuf;

#[derive(Debug)]
pub struct PresetManager {
    presets: Vec<EnvPreset>,
    selected_preset: Option<EnvPreset>,
}

impl PresetManager {
    pub fn new(config: &Value) -> Self {
        let presets = Self::load_presets_from_config(config);
        Self {
            presets,
            selected_preset: None,
        }
    }

    pub fn get_all_presets(&self) -> Vec<EnvPreset> {
        self.presets.clone()
    }

    pub fn get_preset(&self, id: &str) -> Option<&EnvPreset> {
        self.presets.iter().find(|p| p.id == id)
    }

    pub fn select_preset(&mut self, id: &str) -> Result<(), String> {
        if let Some(preset) = self.presets.iter().find(|p| p.id == id).cloned() {
            self.selected_preset = Some(preset);
            Ok(())
        } else {
            Err(format!("No EnvPreset found with id: {}", id))
        }
    }

    pub fn get_selected_preset(&self) -> Option<&EnvPreset> {
        self.selected_preset.as_ref()
    }

    fn load_presets_from_config(config: &Value) -> Vec<EnvPreset> {
        let dirs = config.pointer("/env_presets/directories")
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| expand_env_vars(s))
            .map(PathBuf::from)
            .collect::<Vec<_>>();

        let mut all_presets = Vec::new();

        for dir in dirs {
            match load_env_presets_in_dir(dir.to_str().unwrap_or_default()) {
                Ok(presets) => all_presets.extend(presets),
                Err(err) => {
                    eprintln!("Failed to load presets from {:?}: {}", dir, err);
                }
            }
        }

        all_presets
    }

    pub fn reload(&mut self, config: &Value) {
        self.presets = Self::load_presets_from_config(config);
        // Clear selected preset if it no longer exists
        if let Some(ref selected) = self.selected_preset {
            if !self.presets.iter().any(|p| p.id == selected.id) {
                self.selected_preset = None;
            }
        }
    }
}