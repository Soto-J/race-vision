use shared_memory::{Shmem, ShmemConf, ShmemError};
use std::{collections::HashMap, ffi::c_void};

use crate::sdk::{header::Header, var_buffer::VarBuffer, var_header::VarHeader};

const MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";
const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;

pub struct IRSDK {
    pub parse_yaml_async: bool,
    pub is_initialized: bool,
    pub last_session_info_update: u64,
    // memory/handle fields (Windows-specific will use raw pointers)
    shared_mem: Option<Shmem>,
    header: Option<Header>,
    data_valid_event: Option<*mut c_void>,
    // Variable header caching
    var_headers: Option<Vec<VarHeader>>,
    var_headers_dict: Option<HashMap<String, VarHeader>>,
    var_headers_names: Option<Vec<String>>,
    var_buffer_latest: Option<VarBuffer>,
    // Session and state
    session_info_dict: HashMap<String, serde_yaml::Value>,
    broadcast_msg_id: Option<u32>,
    test_file: Option<String>,
    workaround_connected_state: u16,
}

impl Default for IRSDK {
    fn default() -> Self {
        Self::new()
    }
}

impl IRSDK {
    pub fn new() -> Self {
        Self {
            parse_yaml_async: false,
            is_initialized: false,
            last_session_info_update: 0,
            shared_mem: None,
            header: None,
            data_valid_event: None,
            var_headers: None,
            var_headers_dict: None,
            var_headers_names: None,
            var_buffer_latest: None,
            session_info_dict: HashMap::new(),
            broadcast_msg_id: None,
            test_file: None,
            workaround_connected_state: 0,
        }
    }

    pub fn start_up(
        &self,
        test_file: Option<String>,
        dump_to: Option<String>,
    ) -> Result<(), ShmemError> {
        let shmem = ShmemConf::new()
            .os_id(MEM_MAP_FILE)
            .size(MEM_MAP_FILE_SIZE)
            .open()?;

        // Get a &[u8] view of the memory
        let data = unsafe { shmem.as_slice() };

        // Read first 4 bytes as i32
        let tick_count_bytes = &data[0..4];
        let tick_count = i32::from_le_bytes(tick_count_bytes.try_into().unwrap());

        // Now you can read structured data directly from `data`
        println!("First 16 bytes: {:?}", &data[..16]);

        println!("tick_count = {}", tick_count);

        Ok(())
    }
}
