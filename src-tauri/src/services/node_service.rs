use std::path::PathBuf;
use std::sync::Mutex;
use crate::utils::node::load_nodes_in_dir;
use crate::models::entities::EnvPreset;
use serde_json::Value;
use crate::AppState;
use tauri::State;

pub fn test() {
    load_nodes_in_dir("C:/Users/jmartini/Documents/GitHub/varo-vue/test-data/nodes");
}

// #[tauri::command]
// pub fn get_varo_nodes(state: tauri::State<Mutex<AppState>>) -> Result<Vec<VaroNode>, String> {
//     let mut state = state.lock().unwrap();

//     let base_env = &state.env_vars;
//     let preset_env = state.selected_env_preset
//         .as_ref()
//         .map(|p| &p.env)
//         .unwrap_or(&vec![]);

//     let merged_env = merge_env_vars(base_env, preset_env);

//     let varo_nodes = crate::loaders::varo_node_loader::load_varo_nodes(&merged_env)?;
    
//     // Cache by ID
//     state.varo_nodes.clear();
//     for node in &varo_nodes {
//         state.varo_nodes.insert(node.id.clone(), node.clone());
//     }

//     Ok(varo_nodes)
// }

// #[tauri::command]
// pub fn get_varo_node_by_id(id: String, state: tauri::State<Mutex<AppState>>) -> Option<VaroNode> {
//     let guard = state.lock().unwrap();
//     guard.varo_nodes.get(&id).cloned()
// }