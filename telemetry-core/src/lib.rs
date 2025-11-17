pub mod client;
pub mod dto;
pub mod ibt;
pub mod utils;

pub use client::{IracingClient, error::IRSDKError};
use color_eyre::eyre::{self, Ok};
pub use utils::enums::VarData;

pub struct Reader {
    ir_client: IracingClient,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            ir_client: IracingClient::default(),
        }
    }

    pub async fn init(&mut self) -> eyre::Result<()> {
        self.ir_client.start_up().await?;
        Ok(())
    }

    pub fn get_item(&self, key: &str) -> eyre::Result<VarData> {
        self.ir_client.get_item(key)
    }

    pub fn update_latest_var_buffer(&mut self) -> eyre::Result<()> {
        todo!()
    }
}
