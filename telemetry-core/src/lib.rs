use crate::utils::constants::SIM_STATUS_URL;
pub use client::{IracingClient, error::IRSDKError};
use color_eyre::eyre::{self, Ok};

pub mod client;
pub mod domain;
pub mod dto;
pub mod ibt;
pub mod utils;

// todo!(): This will be the connection between telemetry and frontend
// pub struct Reader {
//     ir_client: IracingClient,
// }

// impl Reader {
//     pub fn new() -> Self {
//         Self {
//             ir_client: IracingClient::default(),
//         }
//     }

//     pub async fn init(&mut self) -> eyre::Result<()> {
//         self.ir_client.start_up().await?;
//         Ok(())
//     }

//     pub fn get_item(&self, key: &str) -> eyre::Result<TelemetryValue> {
//         self.ir_client.get_item(key)
//     }

//     pub fn update_latest_var_buffer(&mut self) -> eyre::Result<()> {
//         todo!()
//     }
// }

pub async fn check_sim_status() -> eyre::Result<()> {
    let res = reqwest::get(SIM_STATUS_URL).await?;
    println!("Sim Status: {:?}", res.status());
    Ok(())
}
