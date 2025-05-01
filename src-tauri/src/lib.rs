// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;

mod models;
mod loaders;
mod utils;

use crate::loaders::varo_node_loader::get_varo_nodes;
use crate::utils::commands::execute_program;

use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextFile {
    path: String,
    modified: String,
    content: String,
}

// change this path accoridng to what you want 
const VARO_PATH: &str = "/Users/macbook/Desktop/Untitled/test-files";

#[tauri::command]
fn get_text_files() -> Result<Vec<TextFile>, String> {
    let path = Path::new(VARO_PATH);
    if !path.exists() || !path.is_dir() {
        return Err(format!("VARO_PATH '{}' is not a valid directory", VARO_PATH));
    }

    let mut text_files = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();

        if file_path.is_file() && 
           file_path.extension().map_or(false, |ext| ext == "txt") {

            let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
            let modified = metadata.modified().map_err(|e| e.to_string())?;
            let datetime: DateTime<Utc> = modified.into();
            let modified_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

            let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;

            text_files.push(TextFile {
                path: file_path.to_string_lossy().to_string(),
                modified: modified_str,
                content,
            });
        }
    }

    Ok(text_files)
}

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
            get_text_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
