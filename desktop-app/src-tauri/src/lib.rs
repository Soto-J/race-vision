use background::register_background_job;
use commands::{
    queries::{page_settings, set_watched_vars, toggle_page, update_setting},
    read_value,
};
use domain::{Database, DomainError};
use shortcuts::register_shortcuts;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, sync::Arc};
use tauri::{App, Manager};
use tokio::sync::RwLock;
use webviews::{register_dashboard, register_widget_webviews};

#[cfg(not(target_os = "windows"))]
use domain::mock_data::telemetry::IracingProvider;
#[cfg(target_os = "windows")]
use telemetry_core::IracingProvider;

mod background;
mod commands;
pub mod domain;
mod shortcuts;
pub mod utils;
mod webviews;

pub const SQLITE_URL: &str = "sqlite:db/app.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), DomainError> {
    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| configure_setup(app))
        .invoke_handler(tauri::generate_handler![
            read_value,
            page_settings,
            toggle_page,
            update_setting,
            set_watched_vars,
        ])
        // .on_window_event(|window, event| {
        // todo!();
        // presist settings on close
        // })
        .run(tauri::generate_context!())
        .map_err(|e| DomainError::Tauri(format!("{e}")))?;

    Ok(())
}

fn configure_setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let provider = Arc::new(
        IracingProvider::new()
            .map_err(|e| DomainError::ProviderInitializationFailed(e.to_string()))?,
    );
    let active_vars = Arc::new(RwLock::new(Vec::new()));
    let edit_mode = RwLock::new(false);

    app.manage(provider.clone());
    app.manage(active_vars.clone());
    app.manage(edit_mode);

    tauri::async_runtime::block_on(async_startup(app))?;

    register_dashboard(app)?;
    register_background_job(app.handle().clone(), provider, active_vars);
    register_shortcuts(app)?;

    Ok(())
}

async fn async_startup(app: &App) -> Result<(), DomainError> {
    let sqlite_pool = get_sqlite_pool().await;

    sqlx::migrate!().run(&sqlite_pool).await?;

    app.manage(Database::new(sqlite_pool));

    register_widget_webviews(app).await?;

    Ok(())
}

pub async fn get_sqlite_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(SQLITE_URL)
        .await
        .expect("failed to create sqlite pool")
}
