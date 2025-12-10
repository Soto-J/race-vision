use crate::{constants::AppWebView, errors::AppError};

use commands::{greet, read_value, set_watched_vars};
use serde::Deserialize;
use std::sync::Arc;
use tauri::{App, AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Wry};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_store::{Store, StoreExt};
use tokio::sync::RwLock;

#[cfg(target_os = "windows")]
use telemetry_core::IracingProvider;

// Mock implementation for non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub mod mock_telemetry {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct IracingProvider;

    impl IracingProvider {
        pub fn new() -> Result<Self, String> {
            Ok(Self)
        }

        pub async fn init(&self) -> Result<(), String> {
            Ok(())
        }

        pub async fn update(&self) -> Result<(), String> {
            Ok(())
        }

        pub async fn read_value(&self, _key: &str) -> Result<TelemetryValue, String> {
            // Return mock data
            Ok(TelemetryValue::F32(vec![0.0]))
        }

        pub async fn read_snapshot(&self, keys: &[String]) -> Result<TelemetrySnapshot, String> {
            let mut data = HashMap::with_capacity(keys.len());
            for key in keys {
                data.insert(key.clone(), TelemetryValue::F32(vec![0.0]));
            }
            Ok(TelemetrySnapshot { data })
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TelemetryValue {
        Chars8(Vec<u8>),
        Bool(Vec<bool>),
        I32(Vec<i32>),
        Bitfield(Vec<u32>),
        F32(Vec<f32>),
        F64(Vec<f64>),
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TelemetrySnapshot {
        pub data: HashMap<String, TelemetryValue>,
    }
}

#[cfg(not(target_os = "windows"))]
use mock_telemetry::IracingProvider;

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

    register_background_job(
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

    register_shortcuts(app)?;

    app.manage(watched_vars);
    app.manage(ir_provider);

    Ok(())
}

fn register_background_job(
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

fn register_shortcuts(app: &App) -> Result<(), tauri_plugin_global_shortcut::Error> {
    let f6 = Shortcut::new(Some(Modifiers::CONTROL), Code::F6);
    let f6_clone = f6.clone();

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts([f6_clone])?
            .with_handler(move |app, shortcut, event| {
                if *shortcut == f6 && event.state == ShortcutState::Pressed {
                    if let Err(e) = app.emit("toggle-edit-mode", ()) {
                        eprintln!("Failed to emit toggle-edit-mode: {e:?}")
                    };
                }
            })
            .build(),
    )?;
    println!("Registering F6 shortcut...");
    Ok(())
}
