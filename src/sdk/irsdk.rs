#![allow(dead_code)]

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

#[cfg(windows)]
use std::{
    collections::HashMap,
    error::{self, Error},
    ffi::{self, CString, OsStr, c_void},
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
            Threading::{self, OpenEventW, SYNCHRONIZATION_ACCESS_RIGHTS},
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
    pub shared_mem_handle: Option<HANDLE>,
    #[cfg(windows)]
    pub shared_mem: Option<Arc<[u8]>>,
    #[cfg(windows)]
    pub data_valid_event: Option<HANDLE>,
    #[cfg(windows)]
    address_space: Option<*mut ffi::c_void>,

    // Variable header caching
    pub header: Option<Header>,
    pub var_headers: Vec<VarHeader>,
    pub var_headers_names: Option<Vec<String>>,
    pub var_headers_hash: HashMap<String, VarHeader>,
    pub var_buffer_latest: Option<VarBuffer>,

    // Session and state
    pub last_session_info_update: u64,
    pub session_info_hash: HashMap<String, VarHeader>,
    pub broadcast_msg_id: Option<u32>,
    pub workaround_connected_state: u16,
}

impl IRSDK {
    pub async fn start_up(
        &mut self,
        test_file: Option<PathBuf>,
        dump_path: Option<PathBuf>,
    ) -> Result<(), Box<dyn error::Error>> {
        match &test_file {
            Some(file) => {
                let file = File::open(file).map_err(|_| IRSDKError::Io)?;

                let mmap = unsafe {
                    MmapOptions::new()
                        .map(&file)
                        .map_err(|e|IRSDKError::FailedToMapView(e.into()))?
                };

                self.shared_mem = Some(Arc::from(mmap.as_ref()));
            }

            None => {
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
                // possibly use invalidhandle
                self.data_valid_event = Some(handle);

                self.wait_for_valid_data_event()
                    .map_err(|_| "Timed out waiting for valid data event")?;

                self.load_meta_data()
                    .map_err(|_| "Failed to load meta data")?;
            }
        };

        if let Some(path) = dump_path {
            if let Some(shared_mem) = &self.shared_mem {
                let mut file = File::create(path)?;

                file.write_all(shared_mem)?;
            }
        }

        self.is_initialized = true;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.is_initialized = false;
        self.last_session_info_update = 0;
        self.shared_mem_handle = None;
        self.shared_mem = None;
        self.header = None;

        // Close OS handles safely
        if let Some(handle) = self.data_valid_event.take() {
            unsafe { CloseHandle(handle).unwrap() };
        }

        self.address_space = None;
        self.var_headers.clear();
        self.var_headers_hash.clear();
        self.var_headers_names = None;
        self.var_buffer_latest = None;
        self.session_info_hash.clear();
        self.broadcast_msg_id = None;
        self.test_file = None;
    }

    fn load_meta_data(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mmap_name = CString::new(MEM_MAP_FILE).map_err(|_| "Failed to create CString")?;

        let handle = unsafe {
            OpenFileMappingA(
                FILE_MAP_READ.0,
                false,
                PCSTR(mmap_name.as_ptr() as *const u8),
            )
        }
        .map_err(|e| IRSDKError::FailedToMapView(e))?;

        let (shared_ptr, address_space) = map_to_address(handle)?;

        // Load shared_mem_handle, shared_mem, & header
        self.shared_mem_handle = Some(handle);
        self.shared_mem = Some(shared_ptr.clone());
        self.header = Some(Header::new(shared_ptr.clone()));

        let header = self.header.as_ref().unwrap();

        if header.status() != StatusField::StatusConnected as i32 {
            unsafe {
                UnmapViewOfFile(address_space)?;
                CloseHandle(handle)?;
            };

            return Err(IRSDKError::NotConnected);
        }

        let mut buffers = header.var_buffers();
        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        // update to latest buffer
        self.var_buffer_latest = if buffers.len() > 1 {
            Some(buffers[1].clone())
        } else {
            buffers.get(0).cloned()
        };

        // Load var_headers_hash
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
                    self.var_headers_hash
                        .insert(var_header_name.to_string(), var_header.clone());
                }

                self.var_headers.push(var_header);
            }
        }

        Ok(())
    }

    pub fn broadcast(&mut self) -> Result<&Broadcast, io::Error> {
        if self.broadcast.is_none() {
            self.broadcast = Some(Broadcast::new()?);
        }

        Ok(self.broadcast.as_ref().unwrap())
    }

    pub fn wait_for_valid_data_event(&self) -> Result<(), IRSDKError> {
        let handle = self.data_valid_event.ok_or(IRSDKError::InvalidHandle)?;

        unsafe {
            let wait_result = Threading::WaitForSingleObject(handle, 32);

            if wait_result != WAIT_OBJECT_0 {
                return Err(IRSDKError::Timeout);
            }

            Ok(())
        }
    }

    pub fn get_item(&self, key: &str) -> Result<VarData, Box<dyn error::Error>> {
        let var_header = self.var_headers_hash.get(key).ok_or(IRSDKError::ItemNotFound)?;

        let buffer = self
            .var_buffer_latest
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory("Buffer not found"))?;

        let memory = buffer.get_memory();

        let base = var_header.offset as usize;
        let count = var_header.count as usize;

        let var_type = IRacingVarType::try_from(var_header.var_type)?;

        // Calculate the total size needed for bounds checking
        let size_per_element = match var_type {
            IRacingVarType::Char | IRacingVarType::Bool => 1,
            IRacingVarType::Int | IRacingVarType::Bitfield | IRacingVarType::Float => 4,
            IRacingVarType::Double => 8,
        };

        let end = base + count * size_per_element;

        // Bounds check to prevent panic
        if end > memory.len() {
            return Err(format!(
                "Variable '{}' data range [{}, {}) exceeds buffer size {}",
                key,
                base,
                end,
                memory.len()
            )
            .into());
        }

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
        };

        Ok(value)
    }

    pub fn unfreeze_var_buffer_latest(&mut self) {
        if let Some(var_buffer) = &mut self.var_buffer_latest {
            var_buffer.unfreeze();
        }
    }

    // Get all buffers and find the most recent one (highest tick_count)
    pub fn freeze_var_buffer_latest(&mut self) -> Result<(), IRSDKError> {
        self.unfreeze_var_buffer_latest();

        self.wait_for_valid_data_event()?;

        let header = self.header.as_ref().ok_or(IRSDKError::InvalidSharedMemory("Header not found"))?;

        let mut buffers = header.var_buffers();

        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        // Take the newest one
        let mut latest = buffers.into_iter().next().ok_or(IRSDKError::InvalidSharedMemory("Buffers not found"))?;

        latest.unfreeze();

        self.var_buffer_latest = Some(latest);
        Ok(())
    }

    fn is_connected(&mut self) -> bool {
        let Some(header) = &self.header else {
            return false;
        };

        let status = header.status();
        let connected = StatusField::StatusConnected as i32;
        let has_session_num = self.var_headers_hash.contains_key("SessionNum");

        self.workaround_connected_state = match (self.workaround_connected_state, status) {
            (0, s) if s != connected => 1,
            (1, _) if !has_session_num || self.test_file.is_some() => 2,
            (2, _) if self.var_headers_hash.contains_key("SessionNum") => 3,
            (_, s) if s == connected => 0,
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

    pub fn parse_to(&self, path: String) -> Result<(), Box<dyn error::Error>> {
        if !self.is_initialized {
            return Err("".into());
        }

        let mut file = File::create(path).map_err(|e| IRSDKError::Io(e))?;

        let memory = self.shared_mem.as_ref().ok_or("No memory found")?;
        let header = self.header.as_ref().ok_or(IRSDKError::InvalidSharedMemory("Header not found"))?;

        let offset = header.session_info_offset() as usize;
        let len = header.session_info_len() as usize;

        // Write session info YAML
        file.write_all(&memory[offset..offset + len])?;
        file.write_all(b"\n")?;

        // Write variable headers
        for (key, var_header) in &self.var_headers_hash {
            let line = format!("{:32}{}\n", key, var_header);

            file.write_all(line.as_bytes())?;
        }

        Ok(())
    }
}

impl Drop for IRSDK {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.address_space {
                let addr = MEMORY_MAPPED_VIEW_ADDRESS { Value: ptr };

                let _ = UnmapViewOfFile(addr);
            }

            if let Some(handle) = self.shared_mem_handle {
                let _ = CloseHandle(handle);
            }

            if let Some(handle) = self.data_valid_event {
                let _ = CloseHandle(handle);
            }
        }
    }
}
