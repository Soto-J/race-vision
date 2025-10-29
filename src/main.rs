use race_vision::{
    sdk::{helpers::check_sim_status, irsdk::IRSDK},
    utils::constants::TelemetryValue,
};
use std::{error, thread::sleep, time::Duration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut irsdk = IRSDK::default();

    println!("Connecting to iRacing...");

    irsdk.start_up(None, None).await.unwrap();

    loop {
        check_sim_status().await?;
        // on each tick we freeze buffer with live telemetry
        // it is optional, but useful if you use vars like CarIdxXXX
        // this way you will have consistent data from those vars inside one tick
        // because sometimes while you retrieve one CarIdxXXX variable
        // another one in next line of code could change
        // to the next iracing internal tick_count
        // and you will get incosistent data

    }
}

// #[tokio::main]
// async fn main() {
//     let mut irsdk = IRSDK::default();

//     println!("Connecting to iRacing...");

//     irsdk.start_up(None, None).await.unwrap();

//     println!("Connected! Starting telemetry stream...\n");
//     println!(
//         "DEBUG: var_buffer_latest is Some: {}",
//         irsdk.var_buffer_latest.is_some()
//     );
//     println!("DEBUG: Number of var_headers: {}", irsdk.var_headers.len());

// Debug: Print first few variable names and their types
// println!("\nAvailable variables (first 20):");
// for (i, var_header) in irsdk.var_headers.iter().take(20).enumerate() {
//     if let Some(name) = var_header.name_str() {
//         println!(
//             "  {} - {} (type: {}, offset: {}, count: {})",
//             i, name, var_header.var_type, var_header.offset, var_header.count
//         );
//     }
// }

// Look for Speed specifically
// println!("\nSearching for Speed variable:");
// if let Some(speed_header) = irsdk.var_headers_dict.get("Speed") {
//     println!(
//         "  Speed - type: {}, offset: {}, count: {}",
//         speed_header.var_type, speed_header.offset, speed_header.count
//     );
// } else {
//     println!("  Speed not found in dictionary!");
// }

// Check buffer details
// if let Some(buf) = &irsdk.var_buffer_latest {
//     println!("\nBuffer details:");
//     println!("  tick_count: {}", buf.tick_count());
//     println!("  buff_offset: {}", buf.buff_offset());

//     let buff_offset = buf.buff_offset() as usize;
//     let memory = buf.get_memory();

//     // Read first 32 bytes at the buff_offset location
//     if buff_offset + 32 <= memory.len() {
//         println!("  First 32 bytes at buff_offset[{}]: ", buff_offset);
//         println!("    {:02X?}", &memory[buff_offset..buff_offset + 32]);

//         // Try to interpret as doubles (SessionTime at offset 0)
//         let session_time_bytes = &memory[buff_offset..buff_offset + 8];
//         let session_time = f64::from_le_bytes(session_time_bytes.try_into().unwrap());
//         println!("  SessionTime (double at offset 0): {}", session_time);
//     }
// }
// println!();

// let telemetry_vars = vec![
//     "Speed",              // Current speed
//     "RPM",                // Engine RPM
//     "Gear",               // Current gear
//     "Throttle",           // Throttle position (0-1)
//     "Brake",              // Brake position (0-1)
//     "SteeringWheelAngle", // Steering angle
//     "LapDistPct",         // Current lap distance percentage
//     "SessionTime",        // Session time
//     "FuelLevel",          // Fuel level
//     "WaterTemp",          // Water temperature
// ];
//     loop {
//         // Wait for iRacing to signal new data is available (blocks until new data or timeout)
//         let _ = irsdk.wait_for_valid_data_event();

//         irsdk.update_var_buf_latest();

//         // Clear screen and reset cursor (works on most terminals)
//         print!("\x1B[2J\x1B[1;1H");
//         println!("=== iRacing Live Telemetry ===\n");

//         if let Some(buf) = &irsdk.var_buffer_latest {
//             println!(
//                 "Buffer tick_count: {} | buff_offset: {}\n",
//                 buf.tick_count(),
//                 buf.buff_offset()
//             );
//         }

//         for var_name in &telemetry_vars {
//             match irsdk.get_item(var_name) {
//                 Ok(value) => {
//                     let formatted_value = format_telemetry_value(&value);
//                     println!("{:20} : {}", var_name, formatted_value);
//                 }
//                 Err(e) => {
//                     println!("{:20} : Error - {}", var_name, e);
//                 }
//             }
//         }

//         println!("\nPress Ctrl+C to exit");

//         // Update every 100ms for smooth real-time updates
//         sleep(Duration::from_millis(100));
//     }
// }
fn format_telemetry_value(value: &race_vision::utils::constants::TelemetryValue) -> String {
    match value {
        TelemetryValue::Float(vals) => {
            if vals.len() == 1 {
                format!("{:.2}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        TelemetryValue::Double(vals) => {
            if vals.len() == 1 {
                format!("{:.2}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        TelemetryValue::Int(vals) => {
            if vals.len() == 1 {
                format!("{}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        TelemetryValue::Bool(vals) => {
            if vals.len() == 1 {
                format!("{}", vals[0])
            } else {
                format!("{:?}", vals)
            }
        }
        TelemetryValue::Bitfield(vals) => format!("{:?}", vals),
        TelemetryValue::Char(vals) => String::from_utf8_lossy(vals).to_string(),
    }
}


async fn check_iracing() {

}