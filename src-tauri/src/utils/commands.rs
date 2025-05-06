use std::collections::HashMap;
use std::process::Command;

#[tauri::command]
pub fn execute_program(
    path: String,
    args: Option<Vec<String>>,
    env_vars: Option<HashMap<String, String>>,
    wait: bool,
) -> Result<(), String> {
    let mut cmd = Command::new(path);

    if let Some(arguments) = args {
        cmd.args(arguments);
    }

    if let Some(env) = env_vars {
        cmd.envs(env);
    }

    if wait {
        // Blocking
        let status = cmd.status().map_err(|e| format!("Launch failed: {}", e))?;
        if !status.success() {
            return Err(format!("Process exited with code: {:?}", status.code()));
        }
    } else {
        // Non-blocking
        cmd.spawn().map_err(|e| format!("Launch failed: {}", e))?;
    }

    Ok(())
}
