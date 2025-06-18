use std::sync::Mutex;
use std::path::PathBuf;
use crate::utils::env::{load_env_presets_in_dir, expand_env_vars};
use crate::models::entities::EnvPreset;
use crate::AppState;
use serde_json::Value;
use tauri::State;

pub fn load_env_presets_from_config(config: &Value) -> Vec<EnvPreset> {
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
                // You can optionally log the error here
                eprintln!("Failed to load presets from {:?}: {}", dir, err);
            }
        }
    }

    all_presets
}

#[tauri::command]
pub fn get_env_presets(state: tauri::State<Mutex<AppState>>) -> Vec<EnvPreset> {
    let state = state.lock().unwrap();
    state.env_presets.clone()
}

#[tauri::command]
pub fn select_env_preset(id: String, state: tauri::State<Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if let Some(preset) = state.env_presets.iter().find(|p| p.id == id).cloned() {
        state.selected_env_preset = Some(preset);
        Ok(())
    } else {
        Err(format!("No EnvPreset found with id: {}", id))
    }
}