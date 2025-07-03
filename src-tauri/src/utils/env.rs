use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde_json::Value;
use crate::models::entities::{EnvVar, EnvPreset};
use crate::utils::hasher::Hasher;

/// Returns current list of environment variables
pub fn get_current_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}

/// Expands `${KEY}` placeholders in a string using values from the provided `vars` map.
/// Returns the updated string with tokens replaced.
pub fn expand_tokens_with_map(input: &str, vars: &HashMap<String, String>) -> String {
    let mut output = input.to_string();
    for (key, value) in vars {
        output = output.replace(&format!("${{{}}}", key), value);
    }
    output
}

/// Expands environment variables in a string using the current process environment.
/// Replaces placeholders like `${VAR_NAME}` with their actual values.
pub fn expand_env_vars(input: &str) -> String {
    let env_map: HashMap<String, String> = env::vars().collect();
    expand_tokens_with_map(input, &env_map)
}

/// Recursively expands environment variables in JSON values in place with custom environment map
pub fn resolve_env_vars_recursive_with_map(value: &mut Value, env_map: &HashMap<String, String>) {
    match value {
        Value::String(s) => {
            *s = expand_tokens_with_map(s, env_map);
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                resolve_env_vars_recursive_with_map(item, env_map);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj.iter_mut() {
                resolve_env_vars_recursive_with_map(v, env_map);
            }
        }
        _ => {}
    }
}

/// Recursively expands environment variables in JSON values in place
/// Uses current process environment by default
pub fn resolve_env_vars_recursive(value: &mut Value) {
    let env_map: HashMap<String, String> = env::vars().collect();
    resolve_env_vars_recursive_with_map(value, &env_map);
}

/// Parses a JSON array into a list of EnvVar objects.
/// Applies environment variable expansion to each field and uses "set" as the default operation.
pub fn parse_env_vars_from_json(env_array: &serde_json::Value) -> Vec<EnvVar> {
    env_array
        .as_array()
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
        .unwrap_or_default()
}

/// Loads a single EnvPreset from a JSON file.
/// Parses fields like name, description, and env variables.
/// If "id" is missing from the file, a fallback ID is generated from the file path.
pub fn load_env_preset_from_file(path: &PathBuf) -> Option<EnvPreset> {
    let content = fs::read_to_string(path).ok()?;
    let json: Value = serde_json::from_str(&content).ok()?;

    let name = json.get("name").and_then(|v| v.as_str())?.to_string();
    let description = json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let env = json.get("env").map(parse_env_vars_from_json).unwrap_or_default();
    let filepath = path.to_str().map(|s| s.to_string());
    let id = json.get("id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Hasher::generate_id_from_path(path));

    Some(EnvPreset {
        id,
        name,
        filepath,
        description,
        env,
    })
}

/// Loads all valid EnvPreset JSON files in the specified directory.
/// Only files with a `.json` extension are considered.
pub fn load_env_presets_in_dir(dir_path: &str) -> Result<Vec<EnvPreset>, String> {
    let mut presets = Vec::new();
    let dir = Path::new(dir_path);

    if !dir.exists() || !dir.is_dir() {
        return Err(format!("Directory does not exist or is not a directory: {}", dir_path));
    }

    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory '{}': {}", dir_path, e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension()
            .and_then(|e| e.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            if let Some(preset) = load_env_preset_from_file(&path) {
                presets.push(preset);
            }
        }
    }

    Ok(presets)
}
