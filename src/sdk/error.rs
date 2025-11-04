use std::{
    error,
    fmt::{self},
};

#[derive(Debug)]
pub enum IRSDKError {
    FailedToOpenMapping(&'static str),
    InvalidSharedMemory(&'static str),
    FailedToMapView(&'static str),
    InvalidHandle,

    // ThreadJoinFailed,
    ItemNotFound,
    NotConnected,
    Timeout,
    NotWindows,
    Io(&'static str),
    Other(&'static str),
}

impl fmt::Display for IRSDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRSDKError::NotWindows => write!(f, "This functionality is only available on Windows"),
            IRSDKError::FailedToOpenMapping(msg) => {
                write!(f, "Failed to open file mapping: {}", msg)
            }
            IRSDKError::FailedToMapView(msg) => write!(f, "Failed to map view of file: {}", msg),
            IRSDKError::InvalidHandle => write!(f, "Invalid handle"),
            IRSDKError::Timeout => write!(f, "Timed out waiting for valid data event"),
            IRSDKError::NotConnected => write!(f, "iRacing is not connected"),
            IRSDKError::InvalidSharedMemory(msg) => write!(f, "Invalid shared memory structure"),
            IRSDKError::Io(msg) => write!(f, "IO error: {}", msg),
            IRSDKError::Other(msg) => write!(f, "{}", msg),
            IRSDKError::ItemNotFound => write!(f, "Item not found"),
        }
    }
}

impl error::Error for IRSDKError {}
