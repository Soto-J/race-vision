pub const DATA_VALID_EVENT_NAME: &str = "Local\\IRSDKDataValidEvent";
pub const MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";
pub const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;
pub const SYNCHRONIZE_ACCESS: u32 = 0x00100000;

pub const VAR_HEADER_SIZE: usize = 144;

pub const SIM_STATUS_URL: &str = "http://127.0.0.1:32034/get_sim_status?object=simStatus";

// iRacing variable type constants
pub const IRACING_CHAR: i32 = 0;
pub const IRACING_BOOL: i32 = 1;
pub const IRACING_INT: i32 = 2;
pub const IRACING_BITFIELD: i32 = 3;
pub const IRACING_FLOAT: i32 = 4;
pub const IRACING_DOUBLE: i32 = 5;
pub const BROADCAST_MSG_NAME: &str = "IRSDK_BROADCASTMSG";

// Enum to represent telemetry values
#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryValue {
    Char(Vec<u8>),
    Bool(Vec<bool>),
    Int(Vec<i32>),
    Bitfield(Vec<u32>),
    Float(Vec<f32>),
    Double(Vec<f64>),
}
