#![allow(unused)]

use std::{error, fmt};

#[derive(Debug)]
pub enum IRSDKError {
    NotWindows,
    FailedToOpenMapping(String),
    FailedToMapView(String),
    InvalidHandle,
    Timeout,
    NotConnected,
    InvalidSharedMemory,
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for IRSDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRSDKError::NotWindows => write!(f, "This functionality is only available on Windows"),
            IRSDKError::FailedToOpenMapping(msg) => {
                write!(f, "Failed to open file mapping: {}", msg)
            }
            IRSDKError::FailedToMapView(msg) => write!(f, "Failed to map view of file: {}", msg),
            IRSDKError::InvalidHandle => write!(f, "Invalid handle"),
            IRSDKError::Timeout => write!(f, "Timed out waiting for valid data event"),
            IRSDKError::NotConnected => write!(f, "iRacing is not connected"),
            IRSDKError::InvalidSharedMemory => write!(f, "Invalid shared memory structure"),
            IRSDKError::Io(e) => write!(f, "IO error: {}", e),
            IRSDKError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl error::Error for IRSDKError {}
