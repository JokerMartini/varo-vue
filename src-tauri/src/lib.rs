// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextFile {
    path: String,
    modified: String,
    content: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_text_files() -> Result<Vec<TextFile>, String> {
    // Get VARO_PATH from environment
    let varo_path = match env::var("VARO_PATH") {
        Ok(path) => path,
        Err(_) => return Err("VARO_PATH environment variable is not set".to_string()),
    };
    
    // Check if path exists and is a directory
    let path = Path::new(&varo_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("VARO_PATH '{}' is not a valid directory", varo_path));
    }
    
    // Collect all .txt files
    let mut text_files = Vec::new();
    
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        
        // Check if it's a .txt file
        if file_path.is_file() && 
           file_path.extension().map_or(false, |ext| ext == "txt") {
            
            // Get last modified time
            let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
            let modified = metadata.modified().map_err(|e| e.to_string())?;
            let datetime: DateTime<Utc> = modified.into();
            let modified_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
            
            // Read content
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_text_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
