use domain::iracing_errors::{ClientError, SharedMemoryError};
use iracing_client::{Client, telemetry::TelemetryValue};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub mod domain;
pub mod dto;
pub mod ibt;
pub mod iracing_client;
pub mod utils;

#[derive(Debug, Clone)]
pub struct IracingProvider {
    ir_client: Arc<RwLock<Client>>,
}

impl IracingProvider {
    pub fn new() -> Result<Self, ClientError> {
        Ok(Self {
            ir_client: Arc::new(RwLock::new(Client::default())),
        })
    }

    pub async fn init(&self) -> Result<(), ClientError> {
        let mut client = self.ir_client.write().await;
        client.init().await?;

        Ok(())
    }

    pub async fn update(&self) -> Result<(), ClientError> {
        let mut client = self.ir_client.write().await;
        client
            .update_latest_var_buffer()
            .map_err(|e| SharedMemoryError::BufferUpdateFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn read_snapshot(&self, keys: &[String]) -> Result<TelemetrySnapshot, ClientError> {
        let client = self.ir_client.read().await;

        let mut data = HashMap::with_capacity(keys.len());

        for key in keys {
            let value = client.read_value(key)?;
            data.insert(key.clone(), value);
        }

        Ok(TelemetrySnapshot { data })
    }

    pub async fn read_value(&self, key: &str) -> Result<TelemetryValue, ClientError> {
        let client = self.ir_client.read().await;
        client.read_value(key)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TelemetrySnapshot {
    pub data: HashMap<String, TelemetryValue>,
}
