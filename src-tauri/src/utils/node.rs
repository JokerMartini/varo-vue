use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use crate::models::varo_node::{VaroNode, Status, Access, Command, EnvVar, EnvPreset};
use crate::utils::hasher::Hasher;
use crate::utils::icon::{resolve_icon_file_path, load_icon_data_uri};
use crate::utils::env::parse_env_vars_from_json;


fn parse_commands_from_json(json: &Value, path: &PathBuf) -> Vec<Command> {
    match json.get("commands") {
        Some(commands_val) => {
            if let Some(commands_arr) = commands_val.as_array() {
                commands_arr.iter().filter_map(|item| {
                    item.as_object().map(|cmd| Command {
                        path: cmd.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        path_type: cmd.get("path_type").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        args: cmd.get("args").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        non_blocking: cmd.get("nonBlocking").and_then(|v| v.as_bool()).unwrap_or(false),
                    })
                }).collect()
            } else {
                eprintln!("Error: 'commands' field must be an array in '{}'", path.display());
                vec![]
            }
        }
        None => {
            eprintln!("Error: Missing 'commands' field in '{}'", path.display());
            vec![]
        }
    }
}

fn parse_status_from_json(json: &Value, path: &PathBuf) -> Option<Status> {
    match json.get("status").and_then(|v| v.as_object()) {
        Some(s) => Some(Status {
            name: s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            color: s.get("color").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }),
        None => {
            eprintln!(
                "Warning: Missing or invalid 'status' block in file '{}', skipping status",
                path.display()
            );
            None
        }
    }
}

fn parse_access_from_json(json: &Value, path: &PathBuf) -> Option<Access> {
    match json.get("access").and_then(|v| v.as_object()) {
        Some(acc) => Some(Access {
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
        }),
        None => {
            eprintln!(
                "Warning: Missing or invalid 'access' block in file '{}', skipping access",
                path.display()
            );
            None
        }
    }
}

pub fn load_node_from_file(path: &PathBuf) -> Result<VaroNode, String> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", path.display(), e);
            return Err(format!("Read error: {}", e));
        }
    };

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON in file {}: {}", path.display(), e))?;

    let name = match json.get("name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => {
            eprintln!(
                "Warning: Missing or invalid 'name' field in file '{}', using 'Untitled'",
                path.display()
            );
            "Untitled".to_string()
        }
    };

    let description = json.get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let filepath = path.to_str().map(|s| s.to_string());

    let id = json.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Hasher::generate_id_from_path(path));

    let category = match json.get("category").and_then(|v| v.as_str()) {
        Some(cat) => cat.to_string(),
        None => {
            eprintln!(
                "Warning: Missing or invalid 'category' field in file '{}', using 'Uncategorized'",
                path.display()
            );
            "Uncategorized".to_string()
        }
    };

    let group_id = match json.get("group_id").and_then(|v| v.as_str()) {
        Some(grp) => grp.to_string(),
        None => {
            eprintln!(
                "Warning: Missing or invalid 'group_id' field in file '{}', using '{}'",
                path.display(), name
            );
            name.to_string()
        }
    };

    let default_for_group = json
        .get("default_for_group")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let status = parse_status_from_json(&json, path);
    let access = parse_access_from_json(&json, path);

    let icon_path = json.get("icon").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let icon_file_path = resolve_icon_file_path(&icon_path);
    let icon_data = icon_file_path
        .as_ref()
        .map(|p| load_icon_data_uri(p))
        .unwrap_or_default();

    let modified = fs::metadata(path)
        .and_then(|meta| meta.modified())
        .unwrap_or(SystemTime::now());
    let date_modified = modified.duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    let env = json.get("env")
        .map(parse_env_vars_from_json)
        .unwrap_or_default();

    let icon_preview = if icon_data.len() > 50 {
        format!("{}...", &icon_data[..50])
    } else {
        icon_data.clone()
    };

    let commands = parse_commands_from_json(&json, path);
    if commands.is_empty() {
        return Err(format!("No valid commands found in '{}'", path.display()));
    }

    println!("Name: {}", name);
    println!("Description: {:?}", description);
    println!("Filepath: {:?}", filepath);
    println!("Id: {:?}", id);
    println!("Category: {:?}", category);
    println!("Group Id: {:?}", group_id);
    println!("Date Modified: {:?}", date_modified);
    println!("Default For Group: {:?}", default_for_group);
    println!("Status: {:?}", status);
    println!("Access: {:?}", access);
    println!("Env: {:?}", env);
    println!("Icon: {:?}", icon_preview);
    println!("Commands: {:?}", commands);
    println!("---");

    let node = VaroNode {
        access,
        category,
        commands,
        date_modified,
        default_for_group,
        description,
        env,
        filepath,
        group_id,
        icon: icon_data,
        id,
        name,
        status,
    };

    Ok(node)
}

pub fn load_nodes_in_dir(dir_path: &str) -> Result<Vec<VaroNode>, String> {
    let mut nodes = Vec::new();
    let dir = Path::new(dir_path);

    if !dir.exists() || !dir.is_dir() {
        return Err(format!("Directory does not exist or is not a directory: {}", dir_path));
    }

    let entries = std::fs::read_dir(dir).map_err(|e| format!("Failed to read directory '{}': {}", dir_path, e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext.eq_ignore_ascii_case("json") {
                    if let Ok(node) = load_node_from_file(&path) {
                        nodes.push(node);
                    }
                }
            }
        }
    }

    println!("Fetched {} nodes", nodes.len());
    Ok(nodes)
}