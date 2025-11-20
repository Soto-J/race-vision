use tauri::State;
use telemetry_core::{iracing_client::telemetry::TelemetryValue, IracingProvider};

#[tauri::command]
async fn read_value(
    key: String,
    // '_ Borrow the state for the duration of this command handler
    state: State<'_, IracingProvider>,
) -> Result<TelemetryValue, String> {
    state.read_value(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(IracingProvider::new().expect("Failed to create Iracing provider"))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, read_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
