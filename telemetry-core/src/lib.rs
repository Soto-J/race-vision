use crate::{
    client::{IracingClient, telemetry::TelemetryValue},
    utils::constants::SIM_STATUS_URL,
};

use color_eyre::eyre::{self, Ok, eyre};
use std::sync::{Arc, RwLock};

pub mod client;
pub mod domain;
pub mod dto;
pub mod ibt;
pub mod utils;

pub struct AppState {
    ir_client: Arc<RwLock<IracingClient>>,
}

impl AppState {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {
            ir_client: Arc::new(RwLock::new(IracingClient::default())),
        })
    }

    pub async fn init(&self) -> eyre::Result<()> {
        let mut client = self
            .ir_client
            .as_ref()
            .write()
            .map_err(|_| eyre!("Telemetry client RwLock was poisoned"))?;

        client.start_up().await?;
        Ok(())
    }

    pub fn read_value(&self, key: &str) -> eyre::Result<TelemetryValue> {
        let client = self
            .ir_client
            .read()
            .map_err(|_| eyre!("Telemetry client RwLock was poisoned"))?;

        client.read_value(key)
    }
}

pub async fn check_sim_status() -> eyre::Result<()> {
    let res = reqwest::get(SIM_STATUS_URL).await?;
    println!("Sim Status: {:?}", res.status());
    Ok(())
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}
