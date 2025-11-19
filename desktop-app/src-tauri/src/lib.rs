use tauri::State;
use telemetry_core::{client::telemetry::TelemetryValue, AppState};

#[tauri::command]
fn get_value(key: String, state: State<AppState>) -> Result<TelemetryValue, String> {
    state.read_value(&key).map_err(|e| e.to_string())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
