use std::process::{Command};
use std::collections::HashMap;

#[tauri::command]
pub fn execute_program(
    path: String, 
    args: Option<Vec<String>>, 
    env_vars: Option<HashMap<String, String>>,
    wait: bool
) -> Result<(), String> {
    println!("[Execute Program] Starting execution:");
    println!("[Execute Program]   Path: '{}'", path);
    println!("[Execute Program]   Args: {:?}", args);
    println!("[Execute Program]   Env vars: {:?}", env_vars);
    println!("[Execute Program]   Wait: {}", wait);

    // Check if the path exists
    let path_exists = std::path::Path::new(&path).exists();
    println!("[Execute Program]   Path exists: {}", path_exists);
    
    if !path_exists {
        println!("[Execute Program]   WARNING: Path does not exist on filesystem");
    }

    let mut cmd = Command::new(&path);
    println!("[Execute Program] Created Command for: {}", path);
    
    if let Some(arguments) = args {
        println!("[Execute Program] Adding arguments: {:?}", arguments);
        cmd.args(arguments);
    } else {
        println!("[Execute Program] No arguments to add");
    }

    if let Some(env) = env_vars {
        println!("[Execute Program] Adding environment variables:");
        for (key, value) in &env {
            println!("[Execute Program]   Setting {}={}", key, value);
        }
        cmd.envs(env);
    } else {
        println!("[Execute Program] No environment variables to add");
    }

    println!("[Execute Program] About to execute command...");

    if wait {
        println!("[Execute Program] Executing in BLOCKING mode");
        // Blocking
        match cmd.status() {
            Ok(status) => {
                println!("[Execute Program] Command completed with status: {:?}", status);
                if !status.success() {
                    let error_msg = format!("Process exited with code: {:?}", status.code());
                    println!("[Execute Program] Error: {}", error_msg);
                    return Err(error_msg);
                } else {
                    println!("[Execute Program] Command executed successfully");
                }
            },
            Err(e) => {
                let error_msg = format!("Launch failed: {}", e);
                println!("[Execute Program] Launch error: {}", error_msg);
                return Err(error_msg);
            }
        }
    } else {
        println!("[Execute Program] Executing in NON-BLOCKING mode");
        // Non-blocking
        match cmd.spawn() {
            Ok(child) => {
                println!("[Execute Program] Process spawned successfully with PID: {}", child.id());
            },
            Err(e) => {
                let error_msg = format!("Launch failed: {}", e);
                println!("[Execute Program] Spawn error: {}", error_msg);
                return Err(error_msg);
            }
        }
    }

    println!("[Execute Program] Execution completed successfully");
    Ok(())
}