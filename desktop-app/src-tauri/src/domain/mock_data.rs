// Mock implementation for non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub mod telemetry {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct IracingProvider;

    impl IracingProvider {
        pub fn new() -> Result<Self, String> {
            Ok(Self)
        }

        pub async fn init(&self) -> Result<(), String> {
            Ok(())
        }

        pub async fn update(&self) -> Result<(), String> {
            Ok(())
        }

        pub async fn read_value(&self, _key: &str) -> Result<TelemetryValue, String> {
            // Return mock data
            Ok(TelemetryValue::F32(vec![0.0]))
        }

        pub async fn read_snapshot(&self, keys: &[String]) -> Result<TelemetrySnapshot, String> {
            let mut data = HashMap::with_capacity(keys.len());
            for key in keys {
                data.insert(key.clone(), TelemetryValue::F32(vec![0.0]));
            }
            Ok(TelemetrySnapshot { data })
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum TelemetryValue {
        Chars8(Vec<u8>),
        Bool(Vec<bool>),
        I32(Vec<i32>),
        Bitfield(Vec<u32>),
        F32(Vec<f32>),
        F64(Vec<f64>),
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct TelemetrySnapshot {
        pub data: HashMap<String, TelemetryValue>,
    }
}
