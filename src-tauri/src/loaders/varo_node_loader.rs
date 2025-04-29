use crate::models::varo_node::{VaroNode, Status, Access, Command, EnvVar, NodeLoadResult};
use crate::utils::{icon_resolver::resolve_icon_path, env::expand_env_vars};
use crate::utils::hasher::Hasher;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
pub async fn get_varo_nodes() -> Result<NodeLoadResult, String> {
    load_varo_nodes()
}

pub fn load_varo_nodes() -> Result<NodeLoadResult, String> {
    let mut warnings = Vec::new();
    let mut nodes = Vec::new();

    let varo_root_str = std::env::var("VARO_PATH")
        .map_err(|_| "Environment variable VARO_PATH is not set.".to_string())?;

    let varo_root = PathBuf::from(&varo_root_str);

    if !varo_root.exists() {
        return Err(format!("VARO_PATH does not exist: {}", varo_root.display()));
    }

    if !varo_root.is_dir() {
        return Err(format!("VARO_PATH is not a directory: {}", varo_root.display()));
    }

    let nodes_dir = varo_root.join("nodes");

    if !nodes_dir.exists() {
        return Err(format!("nodes/ subdirectory is missing under VARO_PATH: {}", nodes_dir.display()));
    }

    if !nodes_dir.is_dir() {
        return Err(format!("nodes/ exists but is not a directory: {}", nodes_dir.display()));
    }

    let entries = fs::read_dir(nodes_dir).map_err(|e| format!("Failed to read nodes dir: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if is_valid_node_file(&path) {
            match parse_varo_node_file(&path) {
                Ok((node, node_warnings)) => {
                    nodes.push(node);
                    warnings.extend(node_warnings);
                }
                Err(e) => warnings.push(format!("Failed to parse {}: {}", path.display(), e)),
            }
        }
    }

    Ok(NodeLoadResult { nodes, warnings })
}

fn is_valid_node_file(path: &Path) -> bool {
    path.is_file() && path.extension().map(|ext| ext.eq_ignore_ascii_case("json")).unwrap_or(false)
}

fn parse_varo_node_file(path: &Path) -> Result<(VaroNode, Vec<String>), String> {
    let mut warnings = Vec::new();

    // Step 1: Read the file content
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

    // Step 2: Parse the JSON
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON in file {}: {}", path.display(), e))?;

    let obj = json.as_object().ok_or_else(|| format!("Root JSON is not an object in {}", path.display()))?;

    // Step 3: Extract fields safely
    let id = Hasher::generate_id_from_path(path);

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

    let group_id = match obj.get("groupId").and_then(|v| v.as_str()) {
        Some(gid) => Some(gid.to_string()),
        None => {
            warnings.push(format!("Node '{}' missing optional 'groupId' field — defaulting to name '{}'", id, name));
            Some(name.clone())
        }
    };

    let filepath = Some(path.to_string_lossy().to_string());

    let default_for_group = Some(
        obj.get("defaultForGroup")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    );

    let description = obj.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    if description.is_none() {
        warnings.push(format!("Node '{}' missing optional 'description' field", id));
    }

    let status = obj.get("status").and_then(|v| v.as_object()).map(|s| {
        Status {
            name: s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            color: s.get("color").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }
    });

    if status.is_none() {
        warnings.push(format!("Node '{}' missing optional 'status' block", id));
    }
    
    let icon_path = obj.get("icon").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let icon = resolve_icon_path(&icon_path, &mut warnings, &id);

    let commands = obj.get("commands")
        .ok_or_else(|| format!("Missing required 'commands' array in {}", path.display()))?
        .as_array()
        .ok_or_else(|| format!("'commands' field must be an array in {}", path.display()))?
        .iter()
        .filter_map(|item| {
            item.as_object().map(|cmd| Command {
                path: cmd.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                path_type: cmd.get("pathType").and_then(|v| v.as_str()).map(|s| s.to_string()),
                args: Some(cmd.get("args").and_then(|v| v.as_str()).unwrap_or("").to_string()),
                non_blocking: Some(cmd.get("nonBlocking").and_then(|v| v.as_bool()).unwrap_or(false)),
            })
        })
        .collect();

    let access = obj.get("access")
        .and_then(|v| v.as_object())
        .map(|acc| Access {
            platforms: acc.get("platforms")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
    
            allow: acc.get("allow")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
    
            deny: acc.get("deny")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
        });
    
    let env = obj.get("env")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_object())
                .map(|env_obj| {
                    let name = env_obj.get("name").and_then(|v| v.as_str()).map(expand_env_vars).unwrap_or_default();
                    let value = env_obj.get("value").and_then(|v| v.as_str()).map(expand_env_vars).unwrap_or_default();
                    let operation = env_obj.get("operation")
                        .and_then(|v| v.as_str())
                        .map(expand_env_vars)
                        .unwrap_or_else(|| "set".to_string());
    
                    EnvVar {
                        name,
                        value,
                        operation: Some(operation),
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
        
    // Step 4: Get the last modified time
    let modified = fs::metadata(path)
        .and_then(|meta| meta.modified())
        .unwrap_or(SystemTime::now());

    let date_modified = modified.duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    Ok((
        VaroNode {
            id,
            name,
            category,
            group_id,
            icon,
            filepath,
            default_for_group,
            description,
            status,
            date_modified,
            commands,
            env,
            access,
        },
        warnings,
    ))
}
