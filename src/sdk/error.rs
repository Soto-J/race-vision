use thiserror::Error;

#[derive(Debug, Error)]
pub enum IRSDKError {
    #[error("File not found")]
    FailedToOpenFile,

    #[error("Failed to open file mapping: {0}")]
    FailedToOpenMapping(String),

    #[error("Failed to map view: {0}")]
    FailedToMapView(String),

    #[error("Item not found")]
    ItemNotFound,

    #[error("Invalid shared memory structure: {0}")]
    InvalidSharedMemory(String),

    #[error("Invalid handle")]
    InvalidHandle,

    #[error("Timed out")]
    Timeout,

    #[error("Not connected to iRacing")]
    NotConnected,

    #[error("This functionality is only available on Windows")]
    NotWindows,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
