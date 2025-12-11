use serde::Serialize;

#[derive(Debug, Serialize, thiserror::Error)]
pub enum AppError {
    #[error("failed to initialize iracing provider: {0}")]
    InitializationFailed(String),

    #[error("failed to update provider: {0}")]
    FailedUpdateProvider(String),

    #[error("telemetry snapshot error: {0}")]
    TelemetrySnapshotError(String),

    #[error("tauri error: {0}")]
    TauriError(String),

    #[error("")]
    ShortcutError(#[from] tauri_plugin_global_shortcut::Error),
}
