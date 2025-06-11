// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;

mod models;
mod loaders;
mod utils;
mod services;

use crate::loaders::varo_node_loader::get_varo_nodes;
use crate::utils::commands::execute_program;
use crate::services::env_preset_service::get_env_presets_for_frontend;
use crate::models::varo_node::EnvPreset;

// --- Public Tauri Commands ---
// Returns current username otherwise returns "Guest"
#[tauri::command]
fn get_os_username() -> String {
    // Try the most common environment variables
    let candidates = ["USERNAME", "USER", "LOGNAME"];

    for var in candidates.iter() {
        if let Ok(val) = env::var(var) {
            if !val.is_empty() {
                return val;
            }
        }
    }

    // If nothing was found, fallback
    "Guest".to_string()
}

#[tauri::command]
fn get_env_presets() -> Result<Vec<EnvPreset>, String> {
    get_env_presets_for_frontend()
}

// Returns the current platform as one of: "win", "mac", "linux"
#[tauri::command]
fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "win"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_platform,
            get_os_username,
            get_varo_nodes,
            execute_program,
            get_env_presets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
