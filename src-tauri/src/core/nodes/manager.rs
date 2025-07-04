use std::collections::HashMap;
use crate::models::entities::{VaroNode, EnvPreset};
use crate::models::errors::{VaroError, VaroResult};
use crate::utils::node::load_nodes_in_dir;
use crate::utils::commands::execute_program;
use crate::utils::platform;

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

    pub fn load_nodes_from_varo_path(&mut self) -> VaroResult<()> {
        // test dir -> "C:/Users/jmartini/Documents/GitHub/varo-vue/test-data/nodes"
        let varo_path = std::env::var("VARO_PATH")
            .map_err(|_| VaroError::node("VARO_PATH environment variable not set"))?;
            
        let nodes_path = format!("{}/nodes", varo_path);
        
        let nodes = load_nodes_in_dir(&nodes_path)
            .map_err(|e| VaroError::node(format!("Failed to load nodes from {}: {}", nodes_path, e)))?;
        
        self.nodes.clear();
        for node in nodes {
            self.nodes.insert(node.id.clone(), node);
        }
        
        Ok(())
    }

    pub fn get_all_nodes(&self) -> Vec<&VaroNode> {
        self.nodes.values().collect()
    }

    pub fn get_node(&self, id: &str) -> Option<&VaroNode> {
        self.nodes.get(id)
    }

    pub fn execute_node(&self, id: &str) -> VaroResult<()> {
        let node = self.nodes.get(id)
            .ok_or_else(|| VaroError::node(format!("Node not found: {}", id)))?;

        if node.commands.is_empty() {
            return Err(VaroError::execution("Node has no commands to execute"));
        }

        // Execute the first command (for now)
        let command = &node.commands[0];
        
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
        ).map_err(|e| VaroError::execution(format!("Failed to execute command: {}", e)))
    }

    pub fn refresh_with_preset(&mut self, _preset: &EnvPreset) -> VaroResult<()> {
        // TODO: Implement preset-based node loading
        // For now, just reload all nodes
        self.load_nodes_from_varo_path()
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn show_node_in_folder(&self, id: &str) -> VaroResult<()> {
        // println!("[Node Manager] Attempting to show node in folder: {}", id);
        
        let node = self.nodes.get(id)
            .ok_or_else(|| VaroError::node(format!("Node not found: {}", id)))?;

        // println!("[Node Manager] Found node: {}", node.name);

        if let Some(filepath) = &node.filepath {
            // println!("[Node Manager] Node filepath: {}", filepath);
            
            let file_path = std::path::Path::new(filepath);

            // Use platform utility to open file manager and select the specific file
            if !platform::open_file_in_folder(file_path) {
                return Err(VaroError::execution("Failed to open file in folder"));
            }
        } else {
            // println!("[Node Manager] Node has no filepath specified");
            return Err(VaroError::execution("Node has no filepath specified"));
        }

        Ok(())
    }
}