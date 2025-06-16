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
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "windows") {
        "win"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}
