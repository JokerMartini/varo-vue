use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use dirs;

mod models;
mod loaders;
mod utils;

use crate::loaders::varo_node_loader::get_varo_nodes;
use crate::utils::commands::execute_program;

use log::{info, warn, error, debug, trace};
use tauri_plugin_log::{Builder as LogBuilder};

// --- LogFile Struct ---
#[derive(Serialize, Deserialize, Debug)]
pub struct LogFile {
    path: String,
    modified: String,
    content: String,
}

#[tauri::command]
fn fetch_log_files() -> Result<Vec<LogFile>, String> {
    info!("Fetching log files...");

    // Use the correct path where Tauri logs are stored
    let log_dir = get_log_dir().map_err(|e| format!("Failed to get log directory: {}", e))?;
    log::info!("Log directory path: {:?}", log_dir);

    let mut logs = Vec::new();

    // Read the log directory for files
    for entry in std::fs::read_dir(log_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.is_file() {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let modified = metadata.modified().map_err(|e| e.to_string())?;
            let modified_time = chrono::DateTime::<chrono::Local>::from(modified);
            let content = std::fs::read_to_string(&path).unwrap_or_default();

            logs.push(LogFile {
                path: path.display().to_string(),
                modified: modified_time.to_string(),
                content,
            });
        }
    }

    Ok(logs)
}

fn get_log_dir() -> Result<PathBuf, String> {
    match dirs::home_dir() {
        Some(home) => {
            let app_name = "com.varo.app";
            let log_dir = match std::env::consts::OS {
                "macos" => home.join(format!("Library/Logs/{}", app_name)),
                "windows" => {
                    let local_app_data = std::env::var("LOCALAPPDATA")
                        .map_err(|e| e.to_string())?;
                    Path::new(&local_app_data)
                        .join(format!("{}/logs", app_name))
                }
                _ => return Err("Unsupported operating system".to_string()),
            };

            if !log_dir.exists() {
                fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
            }

            Ok(log_dir)
        }
        None => Err("Could not determine home directory".to_string()),
    }
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
        .plugin(
            LogBuilder::default()
                .level(log::LevelFilter::Info) // Adjust log level as needed
                .build(),
        )
        .plugin(tauri_plugin_opener::init()) // Other plugin setup
        .invoke_handler(tauri::generate_handler![
            greet,
            get_platform,
            get_os_username,
            get_varo_nodes,
            fetch_log_files,
            execute_program
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}