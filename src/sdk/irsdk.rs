#![allow(unused)]

use crate::{
    sdk::{
        broadcast::Broadcast,
        error::IRSDKError,
        helpers::{check_sim_status, map_to_address},
        memory::{header::Header, var_buffer::VarBuffer, var_header::VarHeader},
        status::StatusField,
    },
    utils::{
        constants::{
            BROADCAST_MSG_NAME, DATA_VALID_EVENT_NAME, MEM_MAP_FILE, SYNCHRONIZE_ACCESS,
            VAR_HEADER_SIZE,
        },
        enums::{IRacingVarType, VarData},
    },
};

use memmap2::MmapOptions;
use std::{
    collections::HashMap,
    error::{self, Error},
    ffi::{CString, OsStr},
    fs::File,
    io::{self, Write},
    os::windows::ffi::OsStrExt,
    path::PathBuf,
    sync::{Arc, OnceLock},
};
use tokio;

#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, LPARAM, WAIT_OBJECT_0, WAIT_TIMEOUT, WPARAM},
        System::{
            Memory::{
                FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, OpenFileMappingA,
                OpenFileMappingW, UnmapViewOfFile,
            },
            Threading::{OpenEventW, SYNCHRONIZATION_ACCESS_RIGHTS, WaitForSingleObject},
        },
        UI::WindowsAndMessaging::{HWND_BROADCAST, RegisterWindowMessageW, SendNotifyMessageW},
    },
    core::{PCSTR, PCWSTR},
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct IRSDK {
    pub parse_yaml_async: bool,
    pub test_file: Option<File>,
    pub is_initialized: bool,
    pub broadcast: Option<Broadcast>,

    // memory/handle fields (Windows-specific will use raw pointers)
    #[cfg(windows)]
    shared_mem_handle: Option<HANDLE>,
    #[cfg(windows)]
    shared_mem: Option<Arc<[u8]>>,
    #[cfg(windows)]
    data_valid_event: Option<HANDLE>,
    #[cfg(windows)]
    address_space: Option<*mut std::ffi::c_void>,

    header: Option<Header>,

    // Variable header caching
    pub var_headers: Vec<VarHeader>,
    pub var_headers_names: Option<Vec<String>>,
    pub var_headers_dict: HashMap<String, VarHeader>,
    pub var_buffer_latest: Option<VarBuffer>,

    // Session and state
    pub last_session_info_update: u64,
    pub session_info_dict: HashMap<String, VarHeader>,
    pub broadcast_msg_id: Option<u32>,
    pub workaround_connected_state: u16,
}

impl IRSDK {
    pub async fn start_up(
        &mut self,
        test_file: Option<PathBuf>,
        dump_path: Option<PathBuf>,
    ) -> Result<(), Box<dyn error::Error>> {
        if test_file.is_none() {
            check_sim_status()
                .await
                .map_err(|_| "Iracing is not connected (HTTP check failed)")?;

            let wide_name: Vec<u16> = OsStr::new(DATA_VALID_EVENT_NAME)
                .encode_wide()
                .chain(Some(0))
                .collect();

            let handle: HANDLE = unsafe {
                OpenEventW(
                    SYNCHRONIZATION_ACCESS_RIGHTS(SYNCHRONIZE_ACCESS), // This requests permission to wait on the event.
                    false, // Don’t inherit this handle by child processes
                    PCWSTR(wide_name.as_ptr()),
                )
            }?;

            self.data_valid_event = Some(handle);
        }

        self.wait_for_valid_data_event()
            .map_err(|_| "Timed out waiting for valid data event")?;

        self.load_meta_data(test_file)
            .map_err(|_| "Failed to load meta data")?;

        if let Some(path) = dump_path {
            if let Some(shared_mem) = &self.shared_mem {
                let mut file = File::create(path)?;

                file.write_all(shared_mem)?;
            }
        }

        self.update_var_buf_latest();
        self.is_initialized = true;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.is_initialized = false;
        self.last_session_info_update = 0;
        self.shared_mem_handle = None;
        self.shared_mem = None;
        self.header = None;
        self.data_valid_event = None;
        self.address_space = None;
        self.var_headers.clear();
        self.var_headers_dict.clear();
        self.var_headers_names = None;
        self.var_buffer_latest = None;
        self.session_info_dict.clear();
        self.broadcast_msg_id = None;
        self.test_file = None;
    }

    fn load_meta_data(&mut self, test_file: Option<PathBuf>) -> Result<(), Box<dyn error::Error>> {
        if self.shared_mem_handle.is_some() {
            return Ok(());
        }

        match test_file {
            Some(file) => {
                let file = File::open(file).map_err(|_| "Failed to open test file")?;

                let mmap = unsafe {
                    MmapOptions::new()
                        .map(&file)
                        .map_err(|_| "Failed to create Mmap")?
                };

                let data: Arc<[u8]> = Arc::from(mmap.as_ref());
                self.shared_mem = Some(data);
            }
            None => {
                let mmap_name =
                    CString::new(MEM_MAP_FILE).map_err(|_| "Failed to create CString")?;

                let handle = unsafe {
                    OpenFileMappingA(
                        FILE_MAP_READ.0,
                        false,
                        PCSTR(mmap_name.as_ptr() as *const u8),
                    )
                }
                .map_err(|_| "Failed to open Mmap")?;

                let (shared_ptr, address_space) = map_to_address(handle)?;

                self.shared_mem_handle = Some(handle);
                self.shared_mem = Some(shared_ptr.clone());
                self.header = Some(Header::new(shared_ptr.clone()));

                // Load VarHeaders
                if let Some(header) = &self.header {
                    if header.status() != StatusField::StatusConnected as i32 {
                        unsafe {
                            UnmapViewOfFile(address_space);
                            CloseHandle(handle);
                        };

                        return Err(
                            "Shared memory exists but sim is not connected (status != 1)".into(),
                        );
                    }

                    let num_vars = header.num_vars().max(0) as usize;
                    let base_offset = header.var_header_offset().max(0) as usize;

                    for i in 0..num_vars {
                        let offset = base_offset + i * VAR_HEADER_SIZE;
                        let end = offset + VAR_HEADER_SIZE;

                        if end > shared_ptr.len() {
                            break;
                        }

                        if let Some(var_header) = VarHeader::from_bytes(&shared_ptr[offset..end]) {
                            if let Some(var_header_name) = var_header.name_str() {
                                self.var_headers_dict
                                    .insert(var_header_name.to_string(), var_header.clone());
                            }

                            self.var_headers.push(var_header);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn get_var_headers(&mut self) -> &Vec<VarHeader> {
        &self.var_headers
    }

    pub fn broadcast(&mut self) -> Result<&Broadcast, io::Error> {
        if self.broadcast.is_none() {
            self.broadcast = Some(Broadcast::new()?);
        }

        Ok(self.broadcast.as_ref().unwrap())
    }

    pub fn wait_for_valid_data_event(&self) -> Result<(), IRSDKError> {
        match self.data_valid_event {
            Some(handle) => unsafe {
                let wait_result = WaitForSingleObject(handle, 32);
                matches!(wait_result, WAIT_OBJECT_0);
                Ok(())
            },
            None => Err(IRSDKError::Timeout),
        }
    }

    pub fn get_item(&self, key: &str) -> Result<VarData, Box<dyn error::Error>> {
        let var_header = self.var_headers_dict.get(key).ok_or("Item not found!")?;

        let buffer = self
            .var_buffer_latest
            .as_ref()
            .ok_or("No variable buffer available")?;

        let memory = buffer.get_memory();
        let base = (buffer.buff_offset() + var_header.offset) as usize;
        let count = var_header.count as usize;

        let var_type = IRacingVarType::try_from(var_header.var_type)?;

        let value = match var_type {
            IRacingVarType::Char => VarData::Chars(memory[base..base + count].to_vec()),
            IRacingVarType::Bool => {
                let bools = memory[base..base + count].iter().map(|&b| b != 0).collect();

                VarData::Bools(bools)
            }
            IRacingVarType::Int => {
                let int = memory[base..base + count * 4]
                    .chunks_exact(4)
                    .map(|b| i32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::Int(int)
            }
            IRacingVarType::Bitfield => {
                let bitfields = memory[base..base + count * 4]
                    .chunks_exact(4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                VarData::Bitfields(bitfields)
            }
            IRacingVarType::Float => {
                let floats = memory[base..base + count * 4]
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                VarData::Floats(floats)
            }
            IRacingVarType::Double => {
                let doubles = memory[base..base + count * 8]
                    .chunks_exact(8)
                    .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                VarData::Doubles(doubles)
            }

            _ => return Err(format!("Unknown variable type: {}", var_header.var_type).into()),
        };

        Ok(value)
    }

    pub fn update_var_buf_latest(&mut self) {
        if let Some(header) = &self.header {
            let mut buffers = header.var_buffers();

            buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

            // Get the 2nd most recent buffer (index 1)
            self.var_buffer_latest = if buffers.len() > 1 {
                Some(buffers[1].clone())
            } else {
                buffers.get(0).cloned()
            };
        }
    }

    pub fn unfreeze_var_buffer_latest(&mut self) {
        if let Some(var_buffer) = &mut self.var_buffer_latest {
            var_buffer.unfreeze();
        }
    }

    pub fn freeze_var_buffer_latest(&mut self) -> Result<(), IRSDKError> {
        &mut self.unfreeze_var_buffer_latest();

        self.wait_for_valid_data_event();

        // Get all buffers and find the most recent one (highest tick_count)
        if let Some(header) = &self.header {
            let mut buffers = header.var_buffers();

            buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

            if let Some(mut latest_buffer) = buffers.into_iter().next() {
                latest_buffer.unfreeze();

                self.var_buffer_latest = Some(latest_buffer);
                return Ok(());
            }
        }

        Err(IRSDKError::Timeout)
    }

    fn is_connected(&mut self) -> bool {
        let Some(header) = &self.header else {
            return false;
        };

        let status = header.status();
        let connected = StatusField::StatusConnected as i32;
        let has_session_num = self.var_headers_dict.contains_key("SessionNum");

        self.workaround_connected_state = match (self.workaround_connected_state, status) {
            (0, s) if s != connected => 1,
            (1, _) if !has_session_num || self.test_file.is_some() => 2,
            (2, _) if self.var_headers_dict.contains_key("SessionNum") => 3,
            (state, s) if s == connected => 0,
            (state, _) => state,
        };

        let is_status_connected = status == connected;
        let is_workaround_connected = self.workaround_connected_state == 3;
        let has_data_source = self.test_file.is_some() || self.data_valid_event.is_some();

        has_data_source && (is_status_connected || is_workaround_connected)
    }

    pub fn session_info_update(&self) -> Option<i32> {
        match &self.header {
            Some(header) => Some(header.session_info_update()),
            None => None,
        }
    }

    fn var_headers_dict(&mut self) -> &HashMap<String, VarHeader> {
        if self.var_headers_dict.is_empty() {
            if let Some(header) = &self.header {
                for var_header in &self.var_headers {
                    let var_header_name = var_header
                        .name_str()
                        .expect("Failed to get var_header name")
                        .to_owned();

                    self.var_headers_dict
                        .insert(var_header_name, var_header.to_owned());
                }
            }
        }

        &self.var_headers_dict
    }
}

impl Drop for IRSDK {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.address_space {
                let addr = MEMORY_MAPPED_VIEW_ADDRESS { Value: ptr };

                UnmapViewOfFile(addr);
            }

            if let Some(handle) = self.shared_mem_handle {
                CloseHandle(handle);
            }
        }
    }
}

#[tokio::test]
async fn test_startup() {
    let mut irsdk = IRSDK::default();
    let response = irsdk.start_up(None, None).await;

    assert!(response.is_ok(), "start_up should succeed.")
}

#[tokio::test]
async fn fails_when_iracing_closed() {
    let mut irsdk = IRSDK::default();
    let response = irsdk.start_up(None, None).await;

    assert!(
        response.is_err(),
        "start_up should fail when iRacing is not running"
    )
}

#[tokio::test]
async fn list_all_available_variables() {
    let mut irsdk = IRSDK::default();

    irsdk
        .start_up(None, None)
        .await
        .expect("Failed to start IRSDK");

    let var_headers = irsdk.get_var_headers();

    println!("\n===== Available Variables ({}) =====", var_headers.len());

    for (i, name) in var_headers.iter().enumerate() {
        if i < 50 {
            println!(
                "\t{}:\t{}",
                i + 1,
                name.name_str().expect("Failed to get name_str")
            );
        }
    }
    println!("  ... and {} more", var_headers.len() - 20);
}

#[tokio::test]
async fn test_data_valid_event() {
    let mut sdk = IRSDK::default();

    sdk.start_up(None, None)
        .await
        .expect("Failed to start IRSDK");

    let data = sdk
        .data_valid_event
        .expect("Failed to get data valid event");

    assert!(
        data.0 != std::ptr::null_mut(),
        "Event handle should be valid"
    );
}
