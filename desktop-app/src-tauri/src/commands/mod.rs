use std::sync::Arc;

use tauri::State;
use telemetry_core::{iracing_client::telemetry::TelemetryValue, IracingProvider};

pub mod queries;

#[tauri::command]
pub async fn read_value(
    key: String,
    state: State<'_, Arc<IracingProvider>>,
) -> Result<TelemetryValue, String> {
    tracing::debug!(key, "read_value invoked");

    state.read_value(&key).await.map_err(|e| e.to_string())
}
