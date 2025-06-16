// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::sync::Mutex;
use std::collections::HashMap;

mod models;
mod loaders;
mod utils;
mod services;

use serde_json::Value;
use tauri::{Builder, Manager};
use crate::loaders::varo_node_loader::get_varo_nodes;
use crate::utils::commands::execute_program;
use crate::services::env_preset_service::load_env_presets_from_config;
use crate::models::varo_node::EnvPreset;
use crate::utils::config::load_config;
use crate::services::system_service::{get_os_username, get_platform};
use crate::utils::env::get_current_env_vars;

// MAIN APP STATE
#[derive(Default)]
struct AppState {
    config: Value,
    env_vars: HashMap<String, String>,
    env_presets: Vec<EnvPreset>,
}

// --- Public Tauri Commands ---
#[tauri::command]
fn get_config(state: tauri::State<Mutex<AppState>>) -> Value {
    let state = state.lock().unwrap();
    state.config.clone()
}

#[tauri::command]
fn reload_config(state: tauri::State<Mutex<AppState>>) -> Result<(), String> {
    let new_config = load_config();
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.config = new_config;
    Ok(())
}

#[tauri::command]
fn get_env_presets(state: tauri::State<'_, Mutex<AppState>>) -> Vec<EnvPreset> {
    let state = state.lock().unwrap();
    state.env_presets.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let config = load_config();
            let env_vars = get_current_env_vars();
            let env_presets = load_env_presets_from_config(&config);

            app.manage(Mutex::new(AppState {
                config,
                env_vars,
                env_presets,
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_platform,
            get_os_username,
            get_varo_nodes,
            execute_program,
            get_env_presets,
            get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
