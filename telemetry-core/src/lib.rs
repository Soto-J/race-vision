use crate::client::{IracingClient, telemetry::TelemetryValue};

use color_eyre::eyre::{self, Ok};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod client;
pub mod domain;
pub mod dto;
pub mod ibt;
pub mod utils;

#[derive(Debug, Clone)]
pub struct IracingProvider {
    ir_client: Arc<RwLock<IracingClient>>,
}

impl IracingProvider {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {
            ir_client: Arc::new(RwLock::new(IracingClient::default())),
        })
    }

    pub async fn init(&self) -> eyre::Result<()> {
        let mut client = self.ir_client.write().await;

        client.start_up().await?;
        Ok(())
    }

    pub async fn read_value(&self, key: &str) -> eyre::Result<TelemetryValue> {
        let client = self.ir_client.read().await;

        client.read_value(key)
    }
}
