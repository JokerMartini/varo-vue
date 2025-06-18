use std::env;

/// Returns os username or Guest
pub fn get_os_username() -> String {
    let candidates = ["USERNAME", "USER", "LOGNAME"];
    for var in candidates.iter() {
        if let Ok(val) = env::var(var) {
            if !val.is_empty() {
                return val;
            }
        }
    }
    "Guest".to_string()
}

/// Returns os name (win, mac, linux, or unknown)
pub fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "win".to_string()
    } else if cfg!(target_os = "macos") {
        "mac".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else {
        "unknown".to_string()
    }
}
