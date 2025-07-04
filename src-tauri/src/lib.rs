use std::sync::Mutex;
use tauri::{Builder, Manager};

mod models;
mod utils;
mod core;

use crate::core::VaroCore;
use crate::utils::commands::execute_program;
use crate::models::errors::VaroError;

// Helper function to convert VaroError to String for Tauri
fn handle_error<T>(result: Result<T, VaroError>) -> Result<T, String> {
    result.map_err(|e| {
        eprintln!("Varo Error: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn get_config(state: tauri::State<Mutex<VaroCore>>) -> Result<serde_json::Value, String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    Ok(state.sync_get_config())
}

#[tauri::command]
fn get_env_presets(state: tauri::State<Mutex<VaroCore>>) -> Result<Vec<crate::models::entities::EnvPreset>, String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    Ok(state.sync_get_all_presets())
}

#[tauri::command]
fn select_env_preset(id: String, state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    handle_error(state.sync_select_preset(&id))
}

#[tauri::command]
fn get_os_username(state: tauri::State<Mutex<VaroCore>>) -> Result<String, String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    Ok(state.get_username().to_string())
}

#[tauri::command]
fn get_platform(state: tauri::State<Mutex<VaroCore>>) -> Result<String, String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    Ok(state.get_platform().to_string())
}

#[tauri::command]
fn reload_config(state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    handle_error(state.sync_reload_config())
}

#[tauri::command]
fn get_nodes(state: tauri::State<Mutex<VaroCore>>) -> Result<Vec<crate::models::entities::VaroNode>, String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    Ok(state.sync_get_all_nodes())
}

#[tauri::command]
fn execute_node(id: String, state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    handle_error(state.sync_execute_node(&id))
}

#[tauri::command]
fn show_node_in_folder(id: String, state: tauri::State<Mutex<VaroCore>>) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Failed to acquire state lock: {}", e))?;
    handle_error(state.sync_show_node_in_folder(&id))
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
            get_nodes,
            execute_node,
            show_node_in_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}