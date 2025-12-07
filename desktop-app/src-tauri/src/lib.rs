use crate::errors::AppError;

use commands::{greet, read_value, set_watched_vars};
use std::sync::Arc;
use tauri::{App, Emitter, Manager};
use telemetry_core::IracingProvider;
use tokio::sync::RwLock;

pub mod commands;
pub mod errors;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), AppError> {
    tauri::Builder::default()
        .setup(|app| setup_config(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_watched_vars,
            read_value
        ])
        .run(tauri::generate_context!())
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    Ok(())
}

pub type WatchedVars = Arc<RwLock<Vec<String>>>;

fn setup_config(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let ir_provider = Arc::new(IracingProvider::new().expect("failed to create provider"));
    let background_provider = ir_provider.clone();

    let watched_vars: WatchedVars = Arc::new(RwLock::new(Vec::new()));
    let background_watched_vars = watched_vars.clone();

    let app_handle = app.handle().clone();

    // Background telemetry loop - Run update on seperate thread
    tauri::async_runtime::spawn(async move {
        // Initialize background provider
        if let Err(e) = background_provider.init().await {
            eprintln!("init failed: {:?}", e);
            return;
        }

        loop {
            if let Err(e) = background_provider.update().await {
                eprintln!("update failed: {:?}", e);
                return;
            };

            let keys = background_watched_vars.read().await.clone();

            if !keys.is_empty() {
                match background_provider.read_snapshot(&keys).await {
                    Ok(snapshot) => {
                        if let Err(e) = app_handle.emit("telemetry-update", snapshot) {
                            eprintln!("emit to frontend failed: {e:?}");
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("failed reading snapshot: {e:?}");
                        return;
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        }
    });

    // Overlay view
    let overlay_view = app
        .get_webview_window("overlay")
        .ok_or_else(|| AppError::InitializationFailed("overlay window not found".into()))?;

    overlay_view.set_ignore_cursor_events(true)?;
    overlay_view.show()?;

    app.manage(watched_vars);
    app.manage(ir_provider);

    Ok(())
}
