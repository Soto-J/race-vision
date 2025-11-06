pub mod flags;
pub mod size;
pub mod telemetry_vars;

pub const DATA_VALID_EVENT_NAME: &str = "Local\\IRSDKDataValidEvent";
pub const MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";

pub const SIM_STATUS_URL: &str = "http://127.0.0.1:32034/get_sim_status?object=simStatus";
pub const BROADCAST_MSG_NAME: &str = "IRSDK_BROADCASTMSG";

pub const SYNCHRONIZE_ACCESS: u32 = 0x00100000;
