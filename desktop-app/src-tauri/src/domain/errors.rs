#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("failed to initialize iracing provider: {0}")]
    ProviderInitializationFailed(String),

    #[error("failed to update provider: {0}")]
    ProviderUpdateFailed(String),

    #[error("telemetry snapshot error: {0}")]
    TelemetrySnapshot(String),

    #[error("tauri error: {0}")]
    Tauri(String),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] sqlx::Error),

    #[error("invalid session")]
    InvalidSession,

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("global shortcut error: {0}")]
    Shortcut(#[from] tauri_plugin_global_shortcut::Error),
}
