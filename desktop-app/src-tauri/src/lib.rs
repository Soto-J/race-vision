use std::sync::Arc;

use tauri::{Manager, State};
use telemetry_core::{iracing_client::telemetry::TelemetryValue, IracingProvider};

#[tauri::command]
async fn read_value(
    key: String,
    // '_ Borrow the state for the duration of this command handler
    state: State<'_, Arc<IracingProvider>>,
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
        .setup(|app| {
            let ir_provider = Arc::new(IracingProvider::new().expect("Failed to create provider"));
            let background_provider = ir_provider.clone();

            tauri::async_runtime::spawn(async move {
                background_provider.init().await.expect("Failed to init");

                loop {
                    background_provider.update().await;
                    tokio::time::sleep(std::time::Duration::from_millis(16)).await;
                }
            });

            app.manage(ir_provider);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, read_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

