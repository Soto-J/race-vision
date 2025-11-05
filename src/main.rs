use race_vision::{
    sdk::{helpers::check_sim_status, irsdk::IRSDK},
    utils::{enums::VarData, telemtry_vars::TelemtryVars},
};

use std::{error, time::Duration};
use tokio::{self, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let telemetry_vars = vec![
        TelemtryVars::SESSION_TIME,
        TelemtryVars::BRAKE,
        TelemtryVars::THROTTLE,
        TelemtryVars::SPEED,
        TelemtryVars::RPM,
        TelemtryVars::GEAR,
        TelemtryVars::STEERING_WHEEL_ANGLE,
    ];

    let mut irsdk = IRSDK::default();

    println!("Connecting to iRacing...");

    irsdk.start_up(None, None).await?;

    loop {
        check_sim_status().await?;

        // on each tick we freeze buffer with live telemetry
        // it is optional, but useful if you use vars like CarIdxXXX
        // this way you will have consistent data from those vars inside one tick
        // because sometimes while you retrieve one CarIdxXXX variable
        // another one in next line of code could change
        // to the next iracing internal tick_count
        // and you will get incosistent data
        irsdk.freeze_latest_var_buffer()?;

        for var_name in telemetry_vars.iter() {
            match irsdk.get_item(var_name) {
                Ok(value) => {
                    let formatted_value = format_telemetry_value(&value);
                    println!("{:20} : {}", var_name, formatted_value);
                }
                Err(e) => {
                    println!("{:20} : Error - {}", var_name, e);
                }
            }
        }

        time::sleep(Duration::from_millis(1000)).await;
        print!("----")
    }
}

fn format_telemetry_value(value: &VarData) -> String {
    match value {
        VarData::Floats(vals) => {
            if vals.len() == 1 {
                format!("{:.2}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        VarData::Doubles(vals) => {
            if vals.len() == 1 {
                format!("{:.2}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        VarData::Int(vals) => {
            if vals.len() == 1 {
                format!("{}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        VarData::Bools(vals) => {
            if vals.len() == 1 {
                format!("{}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        VarData::Bitfields(vals) => format!("{:?}", vals),
        VarData::Chars(vals) => String::from_utf8_lossy(vals).to_string(),
    }
}
