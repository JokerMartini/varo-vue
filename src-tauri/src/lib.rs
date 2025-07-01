use std::sync::Mutex;
use tauri::{Builder, Manager};

mod models;
mod utils;
mod core;

use crate::core::VaroCore;
use crate::utils::commands::execute_program;

// Keep your existing Tauri commands but update them to use VaroCore
#[tauri::command]
fn get_config(state: tauri::State<Mutex<VaroCore>>) -> serde_json::Value {
    let state = state.lock().unwrap();
    state.sync_get_config()
}

#[tauri::command]
fn get_env_presets(state: tauri::State<Mutex<VaroCore>>) -> Vec<crate::models::entities::EnvPreset> {
    let state = state.lock().unwrap();
    state.sync_get_all_presets()
}

#[tauri::command]
fn select_env_preset(id: String, state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    state.sync_select_preset(&id)
}

#[tauri::command]
fn get_os_username(state: tauri::State<Mutex<VaroCore>>) -> String {
    let state = state.lock().unwrap();
    state.get_username().to_string()
}

#[tauri::command]
fn get_platform(state: tauri::State<Mutex<VaroCore>>) -> String {
    let state = state.lock().unwrap();
    state.get_platform().to_string()
}

#[tauri::command]
fn reload_config(state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    state.sync_reload_config()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the new VaroCore instead of old AppState
            let core = VaroCore::new();
            
            app.manage(Mutex::new(core));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_os_username,
            get_platform,
            execute_program,
            get_env_presets,
            select_env_preset,
            get_config,
            reload_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}