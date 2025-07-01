use std::collections::HashMap;
use crate::models::entities::{VaroNode, EnvPreset};
use crate::utils::node::load_nodes_in_dir;
use crate::utils::commands::execute_program;

#[derive(Debug)]
pub struct NodeManager {
    nodes: HashMap<String, VaroNode>,
}

impl NodeManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn load_nodes_from_varo_path(&mut self) -> Result<(), String> {
        // test dir -> "C:/Users/jmartini/Documents/GitHub/varo-vue/test-data/nodes"
        // For now, just call the existing test function
        // TODO: Make this more robust when we improve error handling
        if let Ok(varo_path) = std::env::var("VARO_PATH") {
            let nodes_path = format!("{}/nodes", varo_path);
            match load_nodes_in_dir(&nodes_path) {
                Ok(nodes) => {
                    self.nodes.clear();
                    for node in nodes {
                        self.nodes.insert(node.id.clone(), node);
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Err("VARO_PATH environment variable not set".to_string())
        }
    }

    pub fn get_all_nodes(&self) -> Vec<&VaroNode> {
        self.nodes.values().collect()
    }

    pub fn get_node(&self, id: &str) -> Option<&VaroNode> {
        self.nodes.get(id)
    }

    pub fn execute_node(&self, id: &str) -> Result<(), String> {
        if let Some(node) = self.nodes.get(id) {
            // For now, just execute the first command
            // TODO: Handle multiple commands and different command types
            if let Some(command) = node.commands.first() {
                let args = if command.args.is_empty() {
                    None
                } else {
                    Some(command.args.split_whitespace().map(|s| s.to_string()).collect())
                };

                // Convert node env vars to HashMap
                let env_vars = if node.env.is_empty() {
                    None
                } else {
                    let mut env_map = HashMap::new();
                    for env_var in &node.env {
                        env_map.insert(env_var.name.clone(), env_var.value.clone());
                    }
                    Some(env_map)
                };

                execute_program(
                    command.path.clone(),
                    args,
                    env_vars,
                    !command.non_blocking
                )
            } else {
                Err("Node has no commands to execute".to_string())
            }
        } else {
            Err(format!("Node not found: {}", id))
        }
    }

    pub fn refresh_with_preset(&mut self, _preset: &EnvPreset) -> Result<(), String> {
        // TODO: Implement preset-based node loading
        // For now, just reload all nodes
        self.load_nodes_from_varo_path()
    }
}