use std::path::PathBuf;
use crate::utils::node::load_nodes_in_dir;
use crate::models::entities::EnvPreset;
use serde_json::Value;

pub fn test() {
    load_nodes_in_dir("C:/Users/jmartini/Documents/GitHub/varo-vue/test-data/nodes");
}