//! Background telemetry processing for iRacing data.
//!
//! This module handles continuous polling of iRacing telemetry data in a background task.
//! The telemetry loop runs at ~60 FPS (16ms intervals) and emits updates to the frontend
//! for watched telemetry variables.
#[cfg(not(target_os = "windows"))]
use crate::domain::telemetry::IracingProvider;
#[cfg(target_os = "windows")]
use telemetry_core::IracingProvider;

use crate::utils::constants::TELEMETRY_UPDATE_EVENT;

use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

/// Registers and starts a background task for continuous telemetry polling.
///
/// This function spawns an asynchronous task that:
/// 1. Initializes the iRacing telemetry provider
/// 2. Continuously polls telemetry data every ~16ms (~60 FPS)
/// 3. Reads snapshots of watched telemetry variables
/// 4. Emits `telemetry-update` events to the frontend with the latest data
///
/// The background task will terminate on errors during initialization, updates, or emissions.
///
/// # Arguments
///
/// * `app_handle` - Tauri application handle for emitting events to the frontend
/// * `background_provider` - iRacing telemetry provider instance (platform-specific)
/// * `watched_vars` - Thread-safe list of telemetry variable names to monitor
///
/// # Platform Support
///
/// - **Windows**: Uses the native `telemetry_core::IracingProvider`
/// - **Other platforms**: Uses mock `domain::telemetry::IracingProvider` for development
///
/// # Examples
///
/// ```rust,no_run
/// let provider = Arc::new(IracingProvider::new());
/// let watched = Arc::new(RwLock::new(vec!["Speed".to_string(), "RPM".to_string()]));
/// register_background_job(app_handle, provider, watched);
/// ```
pub fn register_background_job(
    app_handle: AppHandle,
    background_provider: Arc<IracingProvider>,
    watched_vars: Arc<RwLock<Vec<String>>>,
) {
    tracing::info!(phase = "startup", "registering background jobs");

    tauri::async_runtime::spawn(async move {
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
                        if let Err(e) = app_handle.emit(TELEMETRY_UPDATE_EVENT, snapshot) {
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
