use tauri::State;
use std::sync::Mutex;
use crate::AppState;


#[tauri::command]
pub fn get_os_username(state: State<Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.username.clone()
}

#[tauri::command]
pub fn get_platform(state: State<Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.platform.clone()
}