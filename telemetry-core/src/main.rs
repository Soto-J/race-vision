use telemetry_core::{
    client::{IracingClient, check_sim_status, telemetry::TelemetryValue},
    utils::constants::telemetry_vars::TelemetryVars,
};

use color_eyre::eyre;
use std::time::Duration;
use tokio::{self, time};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let telemetry_vars = vec![
        TelemetryVars::SESSION_TIME,
        TelemetryVars::BRAKE,
        TelemetryVars::THROTTLE,
        TelemetryVars::SPEED,
        TelemetryVars::RPM,
        TelemetryVars::GEAR,
        TelemetryVars::STEERING_WHEEL_ANGLE,
    ];

    let mut irsdk = IracingClient::default();

    tracing::debug!("Connecting to iRacing...");

    irsdk.start_up().await?;

    loop {
        check_sim_status().await?;

        // on each tick we freeze buffer with live telemetry
        // it is optional, but useful if you use vars like CarIdxXXX
        // this way you will have consistent data from those vars inside one tick
        // because sometimes while you retrieve one CarIdxXXX variable
        // another one in next line of code could change
        // to the next iracing internal tick_count
        // and you will get incosistent data
        irsdk.update_latest_var_buffer()?;

        for var_name in telemetry_vars.iter() {
            match irsdk.read_value(var_name) {
                Ok(value) => {
                    let formatted_value = TelemetryValue::format(&value);
                    println!("{:20} : {}", var_name, formatted_value);
                }
                Err(e) => {
                    println!("{:20} : Error - {}", var_name, e);
                }
            }
        }

        time::sleep(Duration::from_millis(500)).await;
        print!("----")
    }
}
