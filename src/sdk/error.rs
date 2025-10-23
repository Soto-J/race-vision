#![allow(unused)]

use std::{error, fmt};

#[derive(Debug)]
pub enum IRSDKError {
    NotWindows,
    FailedToOpenMapping(String),
    FailedToMapView(String),
    InvalidHandle,
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
        }
    }
}

impl error::Error for IRSDKError {}
