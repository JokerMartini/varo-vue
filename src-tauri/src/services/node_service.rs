use std::path::PathBuf;
use crate::utils::varo_node::load_nodes_in_dir;
use crate::models::varo_node::EnvPreset;
use serde_json::Value;

pub fn test() {
    load_nodes_in_dir("C:/Users/joker/Documents/GitHub/varo-vue/test-data/nodes");
}