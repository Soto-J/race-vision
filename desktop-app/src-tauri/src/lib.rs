use crate::{constants::AppWebView, errors::AppError};

use commands::{greet, read_value, set_watched_vars};
use serde::Deserialize;
use std::sync::Arc;
use tauri::{App, AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Wry};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_store::{Store, StoreExt};
use telemetry_core::IracingProvider;
use tokio::sync::RwLock;

pub mod commands;
pub mod constants;
pub mod errors;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), AppError> {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| setup_config(app))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
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
    let watched_vars: WatchedVars = Arc::new(RwLock::new(Vec::new()));

    run_background_job(
        app.handle().clone(),
        ir_provider.clone(),
        watched_vars.clone(),
    );

    // Widgets set up
    let store = app
        .store("widget-overlays")
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    let widgets = [
        AppWebView::PEDAL,
        AppWebView::STANDINGS,
        AppWebView::TRACK_MAP,
        AppWebView::RELATIVE,
    ];

    for widget in widgets {
        let layout = load_widget_layout(&store, widget.as_ref());
        init_widget_window(app.handle(), widget.as_ref(), layout)?;
    }

    app.manage(watched_vars);
    app.manage(ir_provider);

    Ok(())
}

fn run_background_job(
    app_handle: AppHandle,
    background_provider: Arc<IracingProvider>,
    watched_vars: WatchedVars,
) {
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

            let keys = watched_vars.read().await.clone();

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
}

#[derive(Debug, Deserialize)]
struct WidgetLayout {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn load_widget_layout(tauri_store: &Store<Wry>, key: &str) -> Option<WidgetLayout> {
    let value = tauri_store.get(key)?;

    serde_json::from_value(value)
        .map_err(|e| eprintln!("Failed to parse widget layout for key {key:?}: {e}"))
        .ok()
}

fn init_widget_window(
    app: &AppHandle,
    label: &str,
    widget_layout: Option<WidgetLayout>,
) -> Result<(), AppError> {
    let window = app
        .get_webview_window(label)
        .ok_or_else(|| AppError::InitializationFailed(format!("window {label} not found")))?;

    if let Some(widget) = widget_layout {
        window
            .set_position(PhysicalPosition::new(widget.x, widget.y))
            .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

        window
            .set_size(PhysicalSize::new(widget.width, widget.height))
            .map_err(|e| AppError::TauriError(format!("{e:?}")))?;
    }

    window
        .show()
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    window
        .set_ignore_cursor_events(true)
        .map_err(|e| AppError::TauriError(format!("{e:?}")))?;

    Ok(())
}

fn register_F6() {}
