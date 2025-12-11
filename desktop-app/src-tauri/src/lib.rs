use background::register_background_job;
use commands::{greet, read_value, set_watched_vars};
use domain::AppError;
use shortcuts::register_shortcuts;
use std::sync::Arc;
use tauri::{App, Manager};
use tokio::sync::RwLock;
use widgets::register_widgets;

#[cfg(not(target_os = "windows"))]
use domain::mock_data::telemetry::IracingProvider;
#[cfg(target_os = "windows")]
use telemetry_core::IracingProvider;

mod background;
mod commands;
pub mod domain;
mod shortcuts;
mod utils;
mod widgets;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), AppError> {
    let mut tauri_builder = tauri::Builder::default();

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[cfg(debug_assertions)]
    {
        tauri_builder = tauri_builder.plugin(devtools);
    }

    tauri_builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| setup_config(app))
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
    let active_vars = Arc::new(RwLock::new(Vec::new()));

    register_background_job(
        app.handle().clone(),
        ir_provider.clone(),
        active_vars.clone(),
    );
    register_widgets(app)?;
    register_shortcuts(app)?;

    let edit_mode = RwLock::new(false);

    app.manage(ir_provider);
    app.manage(active_vars);
    app.manage(edit_mode);

    Ok(())
}
