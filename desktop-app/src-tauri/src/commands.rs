use crate::domain::Settings;

#[cfg(not(target_os = "windows"))]
use crate::domain::telemetry::{IracingProvider, TelemetryValue};

use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

#[cfg(target_os = "windows")]
use telemetry_core::{iracing_client::telemetry::TelemetryValue, IracingProvider};

#[tauri::command]
pub async fn set_watched_vars(
    vars: Vec<String>,
    watched_vars: State<'_, RwLock<Vec<String>>>,
) -> Result<(), String> {
    *watched_vars.write().await = vars;

    Ok(())
}

#[tauri::command]
pub async fn read_value(
    key: String,
    // '_ Borrow the state for the duration of this command handler
    state: State<'_, Arc<IracingProvider>>,
) -> Result<TelemetryValue, String> {
    tracing::debug!(key, "read_value invoked");

    state.read_value(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, SqlitePool>) -> Result<Vec<Settings>, String> {
    sqlx::query_as!(Settings, "SELECT * FROM settings;")
        .fetch_all(&*state)
        .await;

    todo!();
}
