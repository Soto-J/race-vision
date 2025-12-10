use crate::{errors::AppError, WatchedVars};

use std::sync::Arc;
use tauri::State;
use telemetry_core::{iracing_client::telemetry::TelemetryValue, IracingProvider};

#[tauri::command]
pub async fn set_watched_vars(
    vars: Vec<String>,
    watched_vars: State<'_, WatchedVars>,
) -> Result<(), AppError> {
    *watched_vars.write().await = vars;

    Ok(())
}

#[tauri::command]
pub async fn read_value(
    key: String,
    // '_ Borrow the state for the duration of this command handler
    state: State<'_, Arc<IracingProvider>>,
) -> Result<TelemetryValue, String> {
    state.read_value(&key).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_edit_mode() {

}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
