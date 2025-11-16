pub mod client;
pub mod dto;
pub mod ibt;
pub mod utils;

pub use client::{IracingClient, error::IRSDKError};
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

    pub async fn init(&mut self) {
        self.ir_client
            .start_up()
            .await
            .expect("Failed to start telemetry-core client.");
    }
}
