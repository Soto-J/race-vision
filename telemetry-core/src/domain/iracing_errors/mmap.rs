/// Errors that occur during memory mapping operations
#[derive(Debug, thiserror::Error)]
pub enum MMapError {
    // File mapping errors
    #[error("failed to open file mapping: {0}")]
    OpenFileMappingFailed(String),
    #[error("failed to map memory view: {0}")]
    MapViewOfFileFailed(&'static str),
    #[error("MapViewOfFile returned null pointer")]
    MapViewReturnedNull,

    // View errors
    #[error("view pointer not found")]
    ViewPtrNotFound,
    #[error("view address not found")]
    ViewAddressNotFound,

    // Event errors=
    #[error("failed to open event: {0}")]
    OpenEventFailed(String),
    #[error("data valid event not found")]
    DataValidEventNotFound,

    // Snapshot errors
    #[error("snapshot not found")]
    SnapshotNotFound,
}
