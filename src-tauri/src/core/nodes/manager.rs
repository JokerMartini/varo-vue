use std::collections::HashMap;
use crate::models::entities::{VaroNode, EnvPreset};
use crate::models::errors::{VaroError, VaroResult};
use crate::utils::node::load_nodes_in_dir;
use crate::utils::commands::execute_program;
use crate::utils::platform;
use crate::utils::env::{get_current_env_vars, expand_tokens_with_map, get_env_vars_with_preset};

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
        self.load_nodes_from_varo_path_with_preset(None)
    }

    pub fn load_nodes_from_varo_path_with_preset(&mut self, preset: Option<&EnvPreset>) -> VaroResult<()> {
        // Get environment variables with preset precedence
        let env_map = get_env_vars_with_preset(preset);
        
        if let Some(preset) = preset {
            println!("[Node Manager] Using preset '{}' for environment variables", preset.name);
        } else {
            println!("[Node Manager] Using system environment variables only");
        }
        
        // Check if VARO_PATH is available in the combined environment
        let varo_path = match env_map.get("VARO_PATH") {
            Some(path) => path,
            None => {
                // Clear nodes and log warning but don't fail
                self.nodes.clear();
                if let Some(preset) = preset {
                    println!("[Node Manager] Warning: Preset '{}' does not define VARO_PATH and no system VARO_PATH found. No nodes loaded.", preset.name);
                } else {
                    println!("[Node Manager] Warning: VARO_PATH environment variable not set. No nodes loaded.");
                }
                return Ok(());
            }
        };
            
        let nodes_path = format!("{}/nodes", varo_path);
        println!("[Node Manager] Loading nodes from: {}", nodes_path);
        
        // Try to load nodes, but don't fail if the directory doesn't exist
        let nodes = match load_nodes_in_dir(&nodes_path) {
            Ok(nodes) => nodes,
            Err(e) => {
                // Clear nodes and log warning but don't fail
                self.nodes.clear();
                println!("[Node Manager] Warning: Failed to load nodes from {}: {}. No nodes loaded.", nodes_path, e);
                return Ok(());
            }
        };
        
        println!("[Node Manager] Loaded {} nodes from disk", nodes.len());
        
        self.nodes.clear();
        for node in &nodes {
            println!("[Node Manager]   - Loading node: {} ({})", node.name, node.id);
            self.nodes.insert(node.id.clone(), node.clone());
        }
        
        println!("[Node Manager] Node loading complete. Total nodes in memory: {}", self.nodes.len());
        Ok(())
    }

    pub fn get_all_nodes(&self) -> Vec<&VaroNode> {
        let mut nodes: Vec<&VaroNode> = self.nodes.values().collect();
        nodes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        nodes
    }

    pub fn get_node(&self, id: &str) -> Option<&VaroNode> {
        self.nodes.get(id)
    }

    pub fn execute_node(&self, id: &str) -> VaroResult<()> {
        self.execute_node_with_env_expansion(id, None)
    }

    pub fn execute_node_with_env_expansion(&self, id: &str, selected_preset: Option<&EnvPreset>) -> VaroResult<()> {
        println!("[Node Manager] Starting execution for node ID: {}", id);
        
        let node = self.nodes.get(id)
            .ok_or_else(|| VaroError::node(format!("Node not found: {}", id)))?;

        println!("[Node Manager] Found node: {}", node.name);
        println!("[Node Manager] Node has {} commands", node.commands.len());

        if node.commands.is_empty() {
            println!("[Node Manager] Error: Node has no commands to execute");
            return Err(VaroError::execution("Node has no commands to execute"));
        }

        // Build environment variable map for expansion (used for ${VAR} substitution in paths/args)
        // This includes: system vars + preset vars + node vars (in priority order)
        let mut expansion_env = get_current_env_vars();
        println!("[Node Manager] Starting with {} system environment variables", expansion_env.len());
        
        // Add selected preset environment variables if available
        if let Some(preset) = selected_preset {
            println!("[Node Manager] Adding environment variables from selected preset: {}", preset.name);
            for env_var in &preset.env {
                println!("[Node Manager]   Adding preset env: {}={}", env_var.name, env_var.value);
                expansion_env.insert(env_var.name.clone(), env_var.value.clone());
            }
        }

        // Add node-specific environment variables
        for env_var in &node.env {
            println!("[Node Manager]   Adding node env: {}={}", env_var.name, env_var.value);
            expansion_env.insert(env_var.name.clone(), env_var.value.clone());
        }

        // Execute the first command (for now)
        let command = &node.commands[0];
        
        println!("[Node Manager] Executing command:");
        println!("[Node Manager]   Original Path: {}", command.path);
        println!("[Node Manager]   Original Args: '{}'", command.args);
        println!("[Node Manager]   Path type: {}", command.path_type);
        println!("[Node Manager]   Non-blocking: {}", command.non_blocking);

        // Expand environment variables in path and args
        let expanded_path = expand_tokens_with_map(&command.path, &expansion_env);
        let expanded_args_str = expand_tokens_with_map(&command.args, &expansion_env);
        
        println!("[Node Manager]   Expanded Path: {}", expanded_path);
        println!("[Node Manager]   Expanded Args: '{}'", expanded_args_str);
        
        let args = if expanded_args_str.is_empty() {
            println!("[Node Manager]   Parsed args: None (empty)");
            None
        } else {
            let parsed_args: Vec<String> = expanded_args_str.split_whitespace().map(|s| s.to_string()).collect();
            println!("[Node Manager]   Parsed args: {:?}", parsed_args);
            Some(parsed_args)
        };

        // Build environment variables to pass to execute_program (separate from expansion map)
        // These are the actual env vars that will be set in the child process
        // This includes both preset and node environment variables (expanded)
        let mut env_vars_for_execution = HashMap::new();
        
        // Add preset environment variables (expanded)
        if let Some(preset) = selected_preset {
            println!("[Node Manager]   Adding expanded preset environment variables:");
            for env_var in &preset.env {
                let expanded_value = expand_tokens_with_map(&env_var.value, &expansion_env);
                println!("[Node Manager]     preset: {}={}", env_var.name, expanded_value);
                env_vars_for_execution.insert(env_var.name.clone(), expanded_value);
            }
        }
        
        // Add node environment variables (expanded) - these override preset vars if there's a conflict
        if !node.env.is_empty() {
            println!("[Node Manager]   Adding expanded node environment variables:");
            for env_var in &node.env {
                let expanded_value = expand_tokens_with_map(&env_var.value, &expansion_env);
                println!("[Node Manager]     node: {}={}", env_var.name, expanded_value);
                env_vars_for_execution.insert(env_var.name.clone(), expanded_value);
            }
        }
        
        let env_vars = if env_vars_for_execution.is_empty() {
            println!("[Node Manager]   Final environment variables: None");
            None
        } else {
            println!("[Node Manager]   Final environment variables ({} total):", env_vars_for_execution.len());
            for (key, value) in &env_vars_for_execution {
                println!("[Node Manager]     {}={}", key, value);
            }
            Some(env_vars_for_execution)
        };

        let wait_for_completion = !command.non_blocking;
        println!("[Node Manager]   Wait for completion: {}", wait_for_completion);

        // Handle different path types
        match command.path_type.to_lowercase().as_str() {
            "url" => {
                println!("[Node Manager] Detected URL path type, opening in browser");
                if !platform::open_url_in_browser(&expanded_path) {
                    return Err(VaroError::execution("Failed to open URL in browser"));
                }
                println!("[Node Manager] URL opened successfully");
                Ok(())
            },
            _ => {
                // Handle executable paths (rel, abs, or default)
                println!("[Node Manager] Calling execute_program with:");
                println!("[Node Manager]   path: {}", expanded_path);
                println!("[Node Manager]   args: {:?}", args);
                println!("[Node Manager]   env_vars: {:?}", env_vars);
                println!("[Node Manager]   wait: {}", wait_for_completion);

                execute_program(
                    expanded_path,
                    args,
                    env_vars,
                    wait_for_completion
                ).map_err(|e| {
                    println!("[Node Manager] Error from execute_program: {}", e);
                    VaroError::execution(format!("Failed to execute command: {}", e))
                })
            }
        }
    }

    pub fn refresh_with_preset(&mut self, preset: &EnvPreset) -> VaroResult<()> {
        // Load nodes using the preset's environment variables
        self.load_nodes_from_varo_path_with_preset(Some(preset))
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