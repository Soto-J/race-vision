use crate::WatchedVars;

use std::sync::Arc;
use tauri::{AppHandle, Emitter};

#[cfg(not(target_os = "windows"))]
use crate::domain::telemetry::IracingProvider;
#[cfg(target_os = "windows")]
use telemetry_core::IracingProvider;

pub fn register_background_job(
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
