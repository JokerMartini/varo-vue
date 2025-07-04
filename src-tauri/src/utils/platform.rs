use std::env;
use std::path::Path;

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

/// Opens a directory in the system's default file manager
pub fn open_directory_in_folder(dir: &Path) -> bool {
    if !dir.exists() {
        println!("[Platform] Directory does not exist: {}", dir.display());
        return false;
    }

    println!("[Platform] Attempting to open directory: {}", dir.display());
    println!("[Platform] Directory exists: {}", dir.exists());
    println!("[Platform] Is directory: {}", dir.is_dir());

    #[cfg(target_os = "windows")]
    {
        // Convert path to Windows format
        let path_str = dir.to_string_lossy();
        let windows_path = path_str.replace('/', "\\");
        
        println!("[Platform] Original path: {}", path_str);
        println!("[Platform] Windows path: {}", windows_path);
        
        // Try different methods for Windows
        let mut result = std::process::Command::new("explorer")
            .arg(&windows_path)
            .spawn();
            
        // If the first method fails, try with /e flag (opens in folder view)
        if result.is_err() {
            println!("[Platform] First attempt failed, trying with /e flag");
            result = std::process::Command::new("explorer")
                .arg("/e")
                .arg(&windows_path)
                .spawn();
        }
            
        match result {
            Ok(_) => {
                println!("[Platform] Successfully launched explorer");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to launch explorer: {}", e);
                return false;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("[Platform] Opening with macOS 'open' command");
        match std::process::Command::new("open")
            .arg(dir)
            .spawn() {
            Ok(_) => {
                println!("[Platform] Successfully opened finder");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to open finder: {}", e);
                return false;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Try common Linux file managers
        let file_managers = ["xdg-open", "nautilus", "dolphin", "thunar", "pcmanfm"];
        
        println!("[Platform] Trying Linux file managers: {:?}", file_managers);
        
        for manager in &file_managers {
            println!("[Platform] Trying file manager: {}", manager);
            if let Ok(_) = std::process::Command::new(manager)
                .arg(dir)
                .spawn() {
                println!("[Platform] Successfully opened with: {}", manager);
                return true;
            }
        }
        
        println!("[Platform] No suitable file manager found");
        return false;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        println!("[Platform] Unsupported platform: {}", env::consts::OS);
        return false;
    }
}

/// Opens a file in the system's default file manager and selects/highlights it
pub fn open_file_in_folder(file_path: &Path) -> bool {
    if !file_path.exists() {
        println!("[Platform] File does not exist: {}", file_path.display());
        return false;
    }

    println!("[Platform] Attempting to select file in folder: {}", file_path.display());

    #[cfg(target_os = "windows")]
    {
        // Use /select to highlight the specific file
        let path_str = file_path.to_string_lossy();
        let windows_path = path_str.replace('/', "\\");
        
        println!("[Platform] Windows file path: {}", windows_path);
        
        match std::process::Command::new("explorer")
            .arg("/select,")
            .arg(&windows_path)
            .spawn() {
            Ok(_) => {
                println!("[Platform] Successfully opened explorer with file selected");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to launch explorer with /select: {}", e);
                // Fallback to opening the directory
                if let Some(parent) = file_path.parent() {
                    return open_directory_in_folder(parent);
                }
                return false;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Use -R flag to reveal the file in Finder
        println!("[Platform] Opening with macOS 'open -R' command");
        match std::process::Command::new("open")
            .arg("-R")
            .arg(file_path)
            .spawn() {
            Ok(_) => {
                println!("[Platform] Successfully revealed file in finder");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to reveal file in finder: {}", e);
                // Fallback to opening the directory
                if let Some(parent) = file_path.parent() {
                    return open_directory_in_folder(parent);
                }
                return false;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Most Linux file managers don't have a "select file" equivalent
        // Fall back to opening the parent directory
        println!("[Platform] Linux: Opening parent directory (file selection not supported)");
        if let Some(parent) = file_path.parent() {
            return open_directory_in_folder(parent);
        } else {
            println!("[Platform] Unable to get parent directory");
            return false;
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        println!("[Platform] Unsupported platform: {}", env::consts::OS);
        return false;
    }
}

/// Opens a URL in the system's default browser
pub fn open_url_in_browser(url: &str) -> bool {
    println!("[Platform] Attempting to open URL in browser: {}", url);

    #[cfg(target_os = "windows")]
    {
        // Use 'cmd /C start' on Windows
        match std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn() {
            Ok(_) => {
                println!("[Platform] Successfully opened URL in browser via cmd");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to open URL with cmd: {}", e);
                return false;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Use 'open' command on macOS
        match std::process::Command::new("open")
            .arg(url)
            .spawn() {
            Ok(_) => {
                println!("[Platform] Successfully opened URL in browser via open");
                return true;
            },
            Err(e) => {
                println!("[Platform] Failed to open URL with open: {}", e);
                return false;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Try xdg-open first, then common browsers
        let browsers = ["xdg-open", "firefox", "chromium", "google-chrome", "chrome"];
        
        println!("[Platform] Trying Linux browsers: {:?}", browsers);
        
        for browser in &browsers {
            println!("[Platform] Trying browser: {}", browser);
            if let Ok(_) = std::process::Command::new(browser)
                .arg(url)
                .spawn() {
                println!("[Platform] Successfully opened URL with: {}", browser);
                return true;
            }
        }
        
        println!("[Platform] No suitable browser found");
        return false;
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        println!("[Platform] Unsupported platform for URL opening: {}", env::consts::OS);
        return false;
    }
}
