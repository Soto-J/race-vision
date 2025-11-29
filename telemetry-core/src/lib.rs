use crate::{
    domain::iracing_errors::ClientError,
    iracing_client::{Client, telemetry::TelemetryValue},
};

use std::sync::Arc;
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

    pub async fn update(&self) {
        let mut client = self.ir_client.write().await;

        if let Err(e) = client.update_latest_var_buffer() {
            eprintln!("Telemetry update error: {e}");
        }
    }

    pub async fn read_value(&self, key: &str) -> Result<TelemetryValue, ClientError> {
        let client = self.ir_client.read().await;

        client.read_value(key)
    }
}
