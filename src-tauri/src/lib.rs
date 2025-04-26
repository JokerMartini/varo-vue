// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize};
use serde_json::Value;
use base64::engine::general_purpose;
use base64::Engine;

// --- Structs ---

#[derive(Serialize)]
struct ResolvedVaroNode {
    id: String,
    name: String,
    category: Option<String>,
    groupId: Option<String>,
    icon: String, // raw embedded SVG or base64 PNG, or placeholder
    filepath: Option<String>,
    defaultForGroup: Option<bool>,
    description: Option<String>,
    status: Option<ResolvedStatus>,
    // access: Option<ResolvedAccess>,
    // commands: Vec<ResolvedCommand>,
    // env: Vec<ResolvedEnvVar>,
    dateModified: u64, // milliseconds since epoch
}

#[derive(Serialize)]
struct ResolvedStatus {
    name: String,
    color: String,
}

#[derive(Serialize)]
struct ResolvedAccess {
    platforms: Vec<String>,
    allow: Vec<String>,
    deny: Vec<String>,
}

#[derive(Serialize)]
struct ResolvedCommand {
    path: String,
    #[serde(rename = "type")]
    cmd_type: Option<String>,
    args: Option<String>,
    non_blocking: Option<bool>,
}

#[derive(Serialize)]
struct ResolvedEnvVar {
    name: String,
    value: String,
    action: Option<String>,
}

#[derive(Serialize)]
struct NodeLoadResult {
    nodes: Vec<ResolvedVaroNode>,
    warnings: Vec<String>,
}

// --- Public Tauri Commands ---
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
async fn get_varo_nodes() -> Result<NodeLoadResult, String> {
    let mut warnings = Vec::new();
    let mut nodes = Vec::new();

    // Step 1: Check if the environment variable VARO_PATH exists
    let varo_root_str = match env::var("VARO_PATH") {
        Ok(path) => path,
        Err(_) => {
            warnings.push("Environment variable VARO_PATH is not set.".to_string());
            // Early return: cannot continue without VARO_PATH
            return Ok(NodeLoadResult { nodes, warnings });
        }
    };

    let varo_root = PathBuf::from(&varo_root_str);

    // Step 2: Check if VARO_PATH is a directory
    if !varo_root.exists() {
        warnings.push(format!("VARO_PATH path does not exist: {}", varo_root.display()));
        return Ok(NodeLoadResult { nodes, warnings });
    }

    if !varo_root.is_dir() {
        warnings.push(format!("VARO_PATH is not a directory: {}", varo_root.display()));
        return Ok(NodeLoadResult { nodes, warnings });
    }

    // Step 3: Check if the nodes/ subdirectory exists
    let nodes_dir = varo_root.join("nodes");

    if !nodes_dir.exists() {
        warnings.push(format!("nodes/ subdirectory is missing under VARO_PATH: {}", nodes_dir.display()));
        return Ok(NodeLoadResult { nodes, warnings });
    }

    if !nodes_dir.is_dir() {
        warnings.push(format!("nodes/ exists but is not a directory: {}", nodes_dir.display()));
        return Ok(NodeLoadResult { nodes, warnings });
    }

    // Step 4: Iterate all .json files in nodes/
    match std::fs::read_dir(&nodes_dir) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let path = entry.path();
                        if is_valid_node_file(&path) {
                            match parse_varo_node_file(&path) {
                                Ok((node, node_warnings)) => {
                                    nodes.push(node);
                                    warnings.extend(node_warnings);
                                }
                                Err(e) => warnings.push(format!("Failed to parse node file {}: {}", path.display(), e)),
                            }
                        }
                    }
                    Err(e) => {
                        warnings.push(format!("Failed to read an entry in nodes/: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            warnings.push(format!("Failed to read nodes directory: {}", e));
        }
    }

    Ok(NodeLoadResult {
        nodes,
        warnings,
    })
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// --- Core Processing Functions ---
fn is_valid_node_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext.eq_ignore_ascii_case("json"),
        None => false,
    }
}

fn parse_varo_node_file(path: &Path) -> Result<(ResolvedVaroNode, Vec<String>), String> {
    let mut warnings = Vec::new();

    // Step 1: Read the file content
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

    // Step 2: Parse the JSON
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON in file {}: {}", path.display(), e))?;

    let obj = json.as_object().ok_or_else(|| format!("Root JSON is not an object in {}", path.display()))?;

    // Step 3: Extract fields safely
    let id = match obj.get("id").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return Err(format!("Missing or invalid 'id' field in {}", path.display())),
    };

    let name = match obj.get("name").and_then(|v| v.as_str()) {
        Some(v) => v.to_string(),
        None => return Err(format!("Missing or invalid 'name' field in {}", path.display())),
    };

    let category = match obj.get("category").and_then(|v| v.as_str()) {
        Some(cat) => Some(cat.to_string()),
        None => {
            warnings.push(format!("Node '{}' missing optional 'category' field — defaulting to 'Uncategorized'", id));
            Some("Uncategorized".to_string())
        }
    };

    let groupId = match obj.get("groupId").and_then(|v| v.as_str()) {
        Some(gid) => Some(gid.to_string()),
        None => {
            warnings.push(format!("Node '{}' missing optional 'groupId' field — defaulting to name '{}'", id, name));
            Some(name.clone())
        }
    };

    let filepath = Some(path.to_string_lossy().to_string());

    let defaultForGroup = Some(
        obj.get("defaultForGroup")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    );

    let description = obj.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    if description.is_none() {
        warnings.push(format!("Node '{}' missing optional 'description' field", id));
    }

    let status = obj.get("status").and_then(|v| v.as_object()).map(|s| {
        ResolvedStatus {
            name: s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            color: s.get("color").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }
    });

    if status.is_none() {
        warnings.push(format!("Node '{}' missing optional 'status' block", id));
    }
    
    // Mocked placeholder icon for now (we'll replace this later)
    let icon_path = obj.get("icon").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let icon = resolve_icon_path(&icon_path, &mut warnings, &id);

    // Step 4: Get the last modified time
    let modified = fs::metadata(path)
        .and_then(|meta| meta.modified())
        .unwrap_or(SystemTime::now());

    let dateModified = modified.duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    Ok((
        ResolvedVaroNode {
            id,
            name,
            category,
            groupId,
            icon,
            filepath,
            defaultForGroup,
            description,
            status,
            dateModified,
        },
        warnings,
    ))
}

fn resolve_icon_path(raw_icon_path: &str, warnings: &mut Vec<String>, node_id: &str,) -> String {
    if raw_icon_path.is_empty() {
        warnings.push(format!("Node '{}' missing optional 'icon' field", node_id));
        return "".to_string();
    }

    // Resolve any path env variable usage
    let raw_icon_path = expand_env_vars(raw_icon_path);

    // Fetch VARO_PATH internally
    let varo_root = match env::var("VARO_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            warnings.push(format!("Node '{}': VARO_PATH environment variable is not set", node_id));
            return "".to_string();
        }
    };

    let icon_path = PathBuf::from(raw_icon_path);
    let full_icon_path = if icon_path.is_absolute() {
        icon_path
    } else {
        varo_root.join("icons").join(icon_path)
    };

    if !full_icon_path.exists() {
        warnings.push(format!("Node '{}' icon file not found: {}", node_id, full_icon_path.display()));
        return "".to_string();
    }

    match full_icon_path.extension().and_then(|ext| ext.to_str()) {
        Some("svg") => {
            fs::read_to_string(&full_icon_path).unwrap_or_else(|_| {
                warnings.push(format!("Node '{}' failed to read SVG icon: {}", node_id, full_icon_path.display()));
                "".to_string()
            })
        }
        Some(ext) => {
            let mime = format!("image/{}", ext);
            match fs::read(&full_icon_path) {
                Ok(bytes) => format!("data:{};base64,{}", mime, general_purpose::STANDARD.encode(bytes)),
                Err(_) => {
                    warnings.push(format!("Node '{}' failed to read binary icon: {}", node_id, full_icon_path.display()));
                    "".to_string()
                }
            }
        }
        None => {
            warnings.push(format!("Node '{}' icon file has no extension: {}", node_id, full_icon_path.display()));
            "".to_string()
        }
    }
}

fn expand_env_vars(input: &str) -> String {
    let mut out = input.to_string();
    for (key, value) in env::vars() {
        let token = format!("${{{}}}", key);
        out = out.replace(&token, &value);
    }
    out
}

// --- App Entry Point ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_os_username,
            get_varo_nodes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
