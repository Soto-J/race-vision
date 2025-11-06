use crate::{
    sdk::{
        broadcast::Broadcast,
        error::IRSDKError,
        helpers::{check_sim_status, map_to_address},
        memory::{header::Header, var_buffer::VarBuffer, var_header::VarHeader},
        status::StatusField,
    },
    utils::{
        constants::{self},
        enums::{IRacingVarType, VarData},
    },
};

use memmap2::MmapOptions;
use windows::Win32::System::Memory::UnmapViewOfFile;

#[cfg(windows)]
use std::{
    collections::HashMap,
    error::{self},
    ffi::{self},
    fs::File,
    io::{self, Write},
    os::windows::ffi::OsStrExt,
    path::PathBuf,
    sync::Arc,
};
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0},
        System::{
            Memory::{self},
            Threading::{self},
        },
    },
    core::{PCSTR, PCWSTR},
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct IRSDK {
    // memory/handle fields (Windows-specific will use raw pointers)
    #[cfg(windows)]
    file_mapping_handle: Option<HANDLE>,
    #[cfg(windows)]
    pub shared_mem_snapshot: Option<Arc<[u8]>>,
    #[cfg(windows)]
    pub data_valid_event: Option<HANDLE>,
    #[cfg(windows)]
    mapping_view_ptr: Option<*const u8>, // Raw pointer to memory-mapped region for live updates
    #[cfg(windows)]
    mapping_view: Option<*mut ffi::c_void>, // for cleanup/unmapping (stored as the original *mut c_void to match Windows API requirements)

    // Variable header caching
    pub header: Option<Header>,
    pub var_headers: Vec<VarHeader>,
    pub var_header_names: Option<Vec<String>>,
    pub var_headers_hash: HashMap<String, VarHeader>,
    pub latest_var_buffer: Option<VarBuffer>,

    // Session and state
    pub last_session_info_update: u64,
    pub session_info_hash: HashMap<String, VarHeader>,
    pub broadcast_msg_id: Option<u32>,
    pub workaround_connected_state: u16,

    pub is_initialized: bool,
    pub test_file: Option<File>,
    pub parse_yaml_async: bool,
    pub broadcast: Option<Broadcast>,
}

impl IRSDK {
    pub async fn start_up(
        &mut self,
        test_file: Option<PathBuf>,
        dump_path: Option<PathBuf>,
    ) -> Result<(), IRSDKError> {
        match test_file {
            Some(file) => {
                let file =
                    File::open(file).map_err(|_| IRSDKError::Io("Failed to open test file."))?;

                let mmap = unsafe {
                    MmapOptions::new().map(&file).map_err(|_| {
                        IRSDKError::FailedToMapView("Failed to map test file to memory.")
                    })?
                };

                self.shared_mem_snapshot = Some(Arc::from(mmap.as_ref()));
            }

            None => {
                check_sim_status()
                    .await
                    .map_err(|_| IRSDKError::NotConnected)?;

                let wide_name: Vec<u16> = ffi::OsStr::new(constants::DATA_VALID_EVENT_NAME)
                    .encode_wide()
                    .chain(Some(0))
                    .collect();

                let handle: HANDLE = unsafe {
                    Threading::OpenEventW(
                        Threading::SYNCHRONIZATION_ACCESS_RIGHTS(constants::SYNCHRONIZE_ACCESS), // This requests permission to wait on the event.
                        false, // Don’t inherit this handle by child processes
                        PCWSTR(wide_name.as_ptr()),
                    )
                }
                .map_err(|_| IRSDKError::InvalidHandle)?;

                self.data_valid_event = Some(handle);

                self.wait_for_valid_data_event()?;
                self.load_meta_data()?;
            }
        };

        if let Some(path) = dump_path {
            if let Some(shared_mem_snapshot) = &self.shared_mem_snapshot {
                let mut file = File::create(path)
                    .map_err(|_| IRSDKError::Io("Failed to create dump file."))?;

                file.write_all(shared_mem_snapshot)
                    .map_err(|_| IRSDKError::Io("Failed to write to dump file"))?;
            }
        }

        self.is_initialized = true;

        Ok(())
    }

    fn load_meta_data(&mut self) -> Result<(), IRSDKError> {
        let mmap_name = ffi::CString::new(constants::MEM_MAP_FILE)
            .map_err(|_| IRSDKError::Other("Failed to create C String"))?;

        let handle = unsafe {
            Memory::OpenFileMappingA(
                Memory::FILE_MAP_READ.0,
                false,
                PCSTR(mmap_name.as_ptr() as *const u8),
            )
        }
        .map_err(|_| IRSDKError::FailedToMapView("Failed to map view"))?;

        let (mapping_view_ptr, mapping_view) = map_to_address(handle)?;

        // Store the raw pointer and address space
        self.file_mapping_handle = Some(handle);
        self.mapping_view_ptr = Some(mapping_view_ptr);
        self.mapping_view = Some(mapping_view.Value as *mut ffi::c_void);

        // Create initial shared_mem_snapshot from pointer
        let shared_ptr: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(mapping_view_ptr, constants::MEM_MAP_FILE_SIZE);
            Arc::from(slice)
        };

        self.shared_mem_snapshot = Some(shared_ptr.clone());
        self.header = Some(Header::new(shared_ptr.clone()));

        let header = self.header.as_ref().expect("Failed to get header.");

        let mut buffers = header.var_buffers();
        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        // update to latest buffer
        self.latest_var_buffer = if buffers.len() > 1 {
            Some(buffers[1].clone())
        } else {
            buffers.get(0).cloned()
        };

        // Load var_headers_hash & var_header
        let num_vars = header.num_vars().max(0) as usize;
        let base_offset = header.var_header_offset().max(0) as usize;

        for i in 0..num_vars {
            let offset = base_offset + i * constants::VAR_HEADER_SIZE;
            let end = offset + constants::VAR_HEADER_SIZE;

            if end > shared_ptr.len() {
                break;
            }

            let var_header = VarHeader::from_bytes(&shared_ptr[offset..end])
                .ok_or(IRSDKError::InvalidVarHeader("Failed to create var header"))?;

            let var_name = var_header.name_str().ok_or(IRSDKError::InvalidVarHeader(
                "Failed to get var header name",
            ))?;

            self.var_headers_hash
                .insert(var_name.to_string(), var_header.clone());

            self.var_headers.push(var_header);
        }

        Ok(())
    }

    pub fn wait_for_valid_data_event(&self) -> Result<(), IRSDKError> {
        let handle = self.data_valid_event.ok_or(IRSDKError::InvalidHandle)?;

        unsafe {
            let wait_result = Threading::WaitForSingleObject(handle, 32);

            if wait_result == WAIT_OBJECT_0 {
                Ok(())
            } else {
                Err(IRSDKError::Timeout)
            }
        }
    }

    pub fn get_item(&self, key: &str) -> Result<VarData, IRSDKError> {
        let var_header = self
            .var_headers_hash
            .get(key)
            .ok_or(IRSDKError::ItemNotFound)?;

        let buffer = self
            .latest_var_buffer
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory("Buffer not found"))?;

        let memory = buffer.get_memory();

        // When memory is frozen, buff_offset() returns 0, so we need to use the variable offset directly
        // When not frozen, we need to account for the buffer offset in shared memory
        let base = var_header.offset as usize;
        let count = var_header.count as usize;

        let var_type = IRacingVarType::try_from(var_header.var_type)
            .map_err(|_| IRSDKError::InvalidVarType(var_header.var_type))?;

        // Calculate the total size needed for bounds checking
        let size_per_element = match var_type {
            IRacingVarType::Char | IRacingVarType::Bool => 1,
            IRacingVarType::Int | IRacingVarType::Bitfield | IRacingVarType::Float => 4,
            IRacingVarType::Double => 8,
        };

        let end = base + count * size_per_element;

        if end > memory.len() {
            return Err(IRSDKError::InvalidSharedMemory(
                "Variable data range exceeds buffer size",
            ));
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
                    .map(|b| i32::from_le_bytes(b.try_into().unwrap())) /* Unwrap is safe here Since chunks_exact(4) guarantees the size*/
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

    pub fn unfreeze_latest_var_buffer(&mut self) {
        if let Some(var_buffer) = &mut self.latest_var_buffer {
            var_buffer.unfreeze();
        }
    }

    // Get all buffers and find the most recent one (highest tick_count)
    pub fn freeze_latest_var_buffer(&mut self) -> Result<(), IRSDKError> {
        self.unfreeze_latest_var_buffer();
        self.wait_for_valid_data_event()?;

        /* Create a fresh Arc<[u8]> from the mapped pointer to get live data */
        if let Some(ptr) = self.mapping_view_ptr {
            let fresh_shared_mem_snapshot: Arc<[u8]> = unsafe {
                let slice = std::slice::from_raw_parts(ptr, constants::MEM_MAP_FILE_SIZE);
                Arc::from(slice)
            };

            self.shared_mem_snapshot = Some(fresh_shared_mem_snapshot.clone());
            self.header = Some(Header::new(fresh_shared_mem_snapshot));
        }

        let header = self
            .header
            .as_ref()
            .ok_or_else(|| IRSDKError::FailedToMapView("Failed to open map view"))?;

        let mut buffers = header.var_buffers();

        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        // Take the newest one
        let mut latest = buffers
            .into_iter()
            .next()
            .ok_or(IRSDKError::InvalidSharedMemory("Buffers not found"))?;

        latest.freeze();
        self.latest_var_buffer = Some(latest);

        Ok(())
    }

    fn is_connected(&mut self) -> bool {
        let Some(header) = &self.header else {
            return false;
        };

        let session_num_key = "SessionNum";
        let status = header.status();
        let connected = StatusField::StatusConnected as i32;
        let has_session_num = self.var_headers_hash.contains_key(session_num_key);

        self.workaround_connected_state = match (self.workaround_connected_state, status) {
            (0, status) if status != connected => 1,
            (1, _) if !has_session_num || self.test_file.is_some() => 2,
            (2, _) if self.var_headers_hash.contains_key(session_num_key) => 3,
            (_, status) if status == connected => 0,
            (state, _) => state,
        };

        let is_status_connected = status == connected;
        let is_workaround_connected = self.workaround_connected_state == 3;
        let has_data_source = self.test_file.is_some() || self.data_valid_event.is_some();

        has_data_source && (is_status_connected || is_workaround_connected)
    }

    pub fn broadcast(&mut self) -> Result<&Broadcast, io::Error> {
        if self.broadcast.is_none() {
            self.broadcast = Some(Broadcast::new()?);
        }

        Ok(self.broadcast.as_ref().unwrap())
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

        let mut file =
            File::create(path).map_err(|_| IRSDKError::Io("Failed to create parse to file."))?;

        let memory = self.shared_mem_snapshot.as_ref().ok_or("No memory found")?;
        let header = self
            .header
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory("Header not found"))?;

        let offset = header.session_info_offset() as usize;
        let len = header.session_info_len() as usize;

        /* Write session info YAML */
        file.write_all(&memory[offset..offset + len])?;
        file.write_all(b"\n")?;

        /* Write variable headers */
        for (key, var_header) in &self.var_headers_hash {
            let line = format!("{:32}{}\n", key, var_header);

            file.write_all(line.as_bytes())?;
        }

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.is_initialized = false;

        /* Close OS handles safely */
        /* Take guarantees the field becomes None*/
        unsafe {
            if let Some(handle) = self.data_valid_event.take() {
                let _ = CloseHandle(handle);
            }
            if let Some(ptr) = self.mapping_view.take() {
                let addr = Memory::MEMORY_MAPPED_VIEW_ADDRESS { Value: ptr };
                let _ = Memory::UnmapViewOfFile(addr);
            }
            if let Some(handle) = self.file_mapping_handle.take() {
                let _ = CloseHandle(handle);
            }
        }
        self.data_valid_event = None;
        self.mapping_view = None;
        self.file_mapping_handle = None;

        self.last_session_info_update = 0;
        self.shared_mem_snapshot = None;
        self.header = None;
        self.var_headers.clear();
        self.var_headers_hash.clear();
        self.var_header_names = None;
        self.latest_var_buffer = None;
        self.session_info_hash.clear();
        self.broadcast_msg_id = None;
        self.test_file = None;
    }
}

impl Drop for IRSDK {
    fn drop(&mut self) {
        self.shutdown();
    }
}
