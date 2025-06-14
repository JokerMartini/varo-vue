use crate::utils::platform;

#[tauri::command]
pub fn get_os_username() -> String {
    platform::get_os_username()
}

#[tauri::command]
pub fn get_platform() -> &'static str {
    platform::get_platform()
}