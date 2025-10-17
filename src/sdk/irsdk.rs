use crate::sdk::{StatusField, header::Header, var_buffer::VarBuffer, var_header::VarHeader};

use std::{
    collections::HashMap,
    error,
    sync::Arc,
};
use windows::Win32::Foundation::CloseHandle;

#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::HANDLE,
        System::Memory::{FILE_MAP_READ, MapViewOfFile, OpenFileMappingA, UnmapViewOfFile},
    },
    core::PCSTR,
};

const MEM_MAP_FILE: &str = "Local\\IRSDKMemMapFileName";
const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;

const SIM_STATUS_URL: &str = "http://127.0.0.1:32034/get_sim_status?object=simStatus";

#[derive(Debug, Clone, PartialEq)]
pub struct IRSDK {
    pub parse_yaml_async: bool,
    pub is_initialized: bool,
    pub last_session_info_update: u64,

    // memory/handle fields (Windows-specific will use raw pointers)
    #[cfg(windows)]
    shared_mem_handle: Option<HANDLE>,
    #[cfg(windows)]
    shared_mem_ptr: Option<Arc<[u8]>>,
    // #[cfg(not(windows))]
    // shared_mem_handle: Option<()>,
    // #[cfg(not(windows))]
    // shared_mem_ptr: Option<()>,

    header: Option<Header>,
    // Variable header caching
    var_headers: Vec<VarHeader>,
    var_headers_dict: HashMap<String, VarHeader>,
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
            shared_mem_handle: None,
            shared_mem_ptr: None,
            header: None,
            var_headers: Vec::new(),
            var_headers_dict: HashMap::new(),
            var_headers_names: None,
            var_buffer_latest: None,
            session_info_dict: HashMap::new(),
            broadcast_msg_id: None,
            test_file: None,
            workaround_connected_state: 0,
        }
    }

    pub fn start_up(
        &mut self,
        test_file: Option<String>,
        dump_to: Option<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        use std::ffi::CString;

        // Create a null-terminated string for the Windows API
        let mem_map_name = CString::new(MEM_MAP_FILE)?;

        let mem_handle = unsafe {
            OpenFileMappingA(
                FILE_MAP_READ.0,
                false,
                PCSTR(mem_map_name.as_ptr() as *const u8),
            )
        };

        if let Err(_) = mem_handle {
            return Err("Failed to open shared memory. Is iRacing running?".into());
        }

        let shared_mem_handle = mem_handle.unwrap();

        // Map it into our address space
        let mem_view = unsafe {
            MapViewOfFile(
                shared_mem_handle.clone(),
                FILE_MAP_READ,
                0,
                0,
                MEM_MAP_FILE_SIZE,
            )
        };
        let base_ptr = mem_view.Value as *const u8;

        if base_ptr.is_null() {
            unsafe { CloseHandle(shared_mem_handle.clone()) };
            return Err("MapViewOfFile returned null pointer".into());
        }

        let shared_ptr: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(base_ptr, MEM_MAP_FILE_SIZE);
            Arc::from(slice.to_vec())
        };

        // Create Header struct around shared memory pointer (no copies)
        let header = Header::new(shared_ptr.clone());

        if header.status() != StatusField::StatusConnected as i32 {
            unsafe {
                UnmapViewOfFile(mem_view);
                CloseHandle(shared_mem_handle.clone());
            }
            return Err("Shared memory exists but sim is not connected (status != 1)".into());
        }

        println!("Header version: {}", header.version());
        println!("Header status: {}", header.status());
        println!("Header tick_rate: {}", header.tick_rate());
        println!("Header num_buf: {}", header.num_buf());

        self.header = Some(header);
        self.shared_mem_handle = Some(shared_mem_handle);
        self.shared_mem_ptr = Some(shared_ptr);
        self.is_initialized = true;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.is_initialized = false;
        self.last_session_info_update = 0;
        self.shared_mem_handle = None;
        self.shared_mem_ptr = None;
        self.header = None;
        self.var_headers.clear();
        self.var_headers_dict.clear();
        self.var_headers_names = None;
        self.var_buffer_latest = None;
        self.session_info_dict.clear();
        self.broadcast_msg_id = None;
        self.test_file = None;
    }

    fn is_connected(&mut self) -> bool {
        if let Some(header) = &self.header {
            if header.status() == StatusField::StatusConnected as i32 {
                self.workaround_connected_state = 0;
            }

            if self.workaround_connected_state == 0
                && header.status() != StatusField::StatusConnected as i32
            {
                self.workaround_connected_state = 1;
            }

            //  (self.test_file || self.data_valid_event)
            // && (header.status() == StatusField::StatusConnected as i32
            //     || self.workaround_connected_state == 3)
        };

        false
    }

    async fn check_sim_status() -> Result<(), reqwest::Error> {
        let response = reqwest::get(SIM_STATUS_URL).await?;

        println!("{:?}", response);
        Ok(())
    }

    fn var_headers(&self) -> &Vec<VarHeader> {
        &self.var_headers
    }

    fn var_headers_dict(&self) -> &HashMap<String, VarHeader> {
        &self.var_headers_dict
    }
}

#[test]
fn test() {
    let mut irsdk = IRSDK::new();
    let response = irsdk.start_up(None, None);

    assert!(response.is_ok(), "start_up should succeed.")
}

#[test]
fn fails_when_iracing_closed() {
    let mut irsdk = IRSDK::new();
    let response = irsdk.start_up(None, None);

    assert!(
        response.is_err(),
        "start_up should fail when iRacing is not running"
    );
}
