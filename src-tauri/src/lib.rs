// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::sync::Mutex;
use std::collections::HashMap;

mod models;
mod utils;
mod services;

use serde_json::Value;
use tauri::{Builder, Manager};
use crate::utils::commands::execute_program;
use crate::services::env_preset_service::{load_env_presets_from_config, get_env_presets, select_env_preset};
use crate::services::node_service::test;
use crate::models::entities::{EnvPreset, VaroNode};
use crate::utils::config::load_config;
use crate::services::system_service;
use crate::utils::platform;
use crate::utils::env::get_current_env_vars;

// MAIN APP STATE
#[derive(Default)]
pub struct AppState {
    pub username: String,
    pub platform: String,
    pub config: Value,
    pub env_vars: HashMap<String, String>,
    pub env_presets: Vec<EnvPreset>,
    pub selected_env_preset: Option<EnvPreset>,
    pub varo_nodes: HashMap<String, VaroNode>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let username = platform::get_os_username();
            let platform = platform::get_platform();
            let config = load_config();
            let env_vars = get_current_env_vars();
            let env_presets = load_env_presets_from_config(&config);

            test();

            app.manage(Mutex::new(AppState {
                username,
                platform,
                config,
                env_vars,
                env_presets,
                selected_env_preset: None,
                varo_nodes: HashMap::new(),
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            system_service::get_os_username,
            system_service::get_platform,
            execute_program,
            get_env_presets,
            select_env_preset,
            get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
