use crate::{
    client::{
        broadcast::Broadcast,
        error::IRSDKError,
        helpers::{check_sim_status, slice_var_bytes},
        memory::{header::Header, var_buffer::VarBuffer, var_header::VarHeader},
        session_state::SessionState,
    },
    utils::{
        constants::{self, size},
        enums::{IRacingVarType, VarData},
    },
};

use color_eyre::eyre::{self};
use memmap2::MmapOptions;
use windows::Win32::System::Memory::{FILE_MAP_READ, MapViewOfFile};

#[cfg(windows)]
use std::{
    collections::HashMap, error, ffi, fs::File, io::Write, os::windows::ffi::OsStrExt,
    path::PathBuf, sync::Arc,
};
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0},
        System::{Memory, Threading},
    },
    core::{PCSTR, PCWSTR},
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct IracingClient {
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
    pub var_header_names: Vec<String>,
    pub var_headers_hash: HashMap<String, VarHeader>,
    pub latest_var_buffer: Option<VarBuffer>,

    pub session_state: SessionState,

    pub is_initialized: bool,
    pub test_file: Option<File>,
    pub parse_yaml_async: bool,
    pub broadcast: Option<Broadcast>,
}

impl IracingClient {
    pub async fn start_up(
        &mut self,
        test_file: Option<PathBuf>,
        dump_path: Option<PathBuf>,
    ) -> Result<(), IRSDKError> {
        match test_file {
            Some(file) => {
                let file = File::open(file).map_err(|_| {
                    IRSDKError::UnexpectedError(eyre::eyre!("Failed to open test file."))
                })?;

                let mmap = unsafe { MmapOptions::new().map(&file) }.map_err(|e| {
                    IRSDKError::FailedToMapView(format!("Failed to map test file to memory: {}", e))
                })?;

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

                /*  Opens the iRacing "data valid" Windows event.
                ///
                /// This event is signaled by iRacing every time telemetry updates.
                /// We request only `SYNCHRONIZE` access so the client can *wait* on
                /// the event (e.g. via `WaitForSingleObject`) but not modify it.
                ///
                /// # Safety
                /// This calls the raw Windows API `OpenEventW` and uses a raw UTF-16
                /// string pointer. The caller must ensure:
                /// - `wide_name` is a valid, null-terminated UTF-16 buffer
                /// - the event name matches an existing iRacing event
                /// - the returned handle is closed when no longer needed
                ///
                /// Returns a `HANDLE` which may be invalid if the event does not exist.
                 */
                let handle: HANDLE = unsafe {
                    Threading::OpenEventW(
                        Threading::SYNCHRONIZATION_ACCESS_RIGHTS(constants::SYNCHRONIZE_ACCESS), // This requests permission to wait on the event.
                        false, // Don’t inherit this handle by child processes
                        PCWSTR(wide_name.as_ptr()),
                    )
                }
                .map_err(|_| IRSDKError::InvalidHandle)?;

                self.data_valid_event = Some(handle);

                self.load_meta_data()?;
            }
        };

        if let Some(path) = dump_path {
            if let Some(shared_mem_snapshot) = &self.shared_mem_snapshot {
                let mut file = File::create(path).map_err(|_| {
                    IRSDKError::UnexpectedError(eyre::eyre!("Failed to create dump file."))
                })?;

                file.write_all(shared_mem_snapshot).map_err(|_| {
                    IRSDKError::UnexpectedError(eyre::eyre!("Failed to write to dump file"))
                })?;
            }
        }

        self.is_initialized = true;

        Ok(())
    }

    fn load_meta_data(&mut self) -> Result<(), IRSDKError> {
        self.wait_for_valid_data_event()?;
        self.open_memory_map()?;
        self.parse_headers()?;

        Ok(())
    }

    // opens a handle to a memory-mapped file and maps it into the process's address space
    fn open_memory_map(&mut self) -> eyre::Result<(), IRSDKError> {
        let mmap_name = ffi::CString::new(constants::MEM_MAP_FILE)
            .map_err(|_| IRSDKError::UnexpectedError(eyre::eyre!("Failed to create C String")))?;

        let handle = unsafe {
            Memory::OpenFileMappingA(
                Memory::FILE_MAP_READ.0,
                false,
                PCSTR(mmap_name.as_ptr() as *const u8),
            )
        }
        .map_err(|e| IRSDKError::FailedToMapView(format!("Failed to open map view: {e}")))?;

        // Map memory to address
        let memory_map_view =
            unsafe { MapViewOfFile(handle.clone(), FILE_MAP_READ, 0, 0, size::MEM_MAP_FILE_SIZE) };

        let mapping_view_ptr = memory_map_view.Value as *const u8;

        if mapping_view_ptr.is_null() {
            let _ = unsafe { CloseHandle(handle.clone()) };

            return Err(IRSDKError::FailedToMapView(
                "Map view of file returned null pointer".to_owned(),
            ));
        }

        // Store the raw pointer and address space
        self.file_mapping_handle = Some(handle);
        self.mapping_view_ptr = Some(mapping_view_ptr);
        self.mapping_view = Some(memory_map_view.Value as *mut ffi::c_void);

        // Create initial shared_mem_snapshot from pointer
        let shared_ptr: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(mapping_view_ptr, size::MEM_MAP_FILE_SIZE);
            Arc::from(slice)
        };

        self.shared_mem_snapshot = Some(shared_ptr);

        Ok(())
    }

    fn parse_headers(&mut self) -> Result<(), IRSDKError> {
        let memory = self
            .shared_mem_snapshot
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory(
                "Iracing shared memory not found".to_owned(),
            ))?;

        // load header
        self.header = Some(Header::new(memory.clone()));

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
            let offset = base_offset + i * size::VAR_HEADER_SIZE;
            let end = offset + size::VAR_HEADER_SIZE;

            if end > memory.len() {
                break;
            }

            let var_header = VarHeader::from_bytes(&memory[offset..end]).ok_or(
                IRSDKError::InvalidVarHeader("Failed to create var header".to_owned()),
            )?;

            let var_name = var_header.name_str().ok_or(IRSDKError::InvalidVarHeader(
                "Failed to get var header name".to_owned(),
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

        let latest_buffer =
            self.latest_var_buffer
                .as_ref()
                .ok_or(IRSDKError::InvalidSharedMemory(
                    "Buffer not found".to_owned(),
                ))?;

        // When memory is frozen, buff_offset() returns 0, so we need to use the variable offset directly
        // When not frozen, we need to account for the buffer offset in shared memory
        let (bytes, var_type) = slice_var_bytes(
            latest_buffer.get_memory(),
            var_header.offset as usize,
            var_header.count as usize,
            var_header.var_type,
        )?;

        let value = match var_type {
            IRacingVarType::Char8 => VarData::Chars8(bytes.to_vec()),
            IRacingVarType::Bool => {
                let bools = bytes.iter().map(|&b| b != 0).collect();

                VarData::Bool(bools)
            }
            IRacingVarType::I32 => {
                let int = bytes
                    .chunks_exact(4)
                    .map(|b| i32::from_le_bytes(b.try_into().unwrap())) /* Unwrap is safe here Since chunks_exact(4) guarantees the size*/
                    .collect();

                VarData::I32(int)
            }
            IRacingVarType::Bitfield => {
                let bitfields = bytes
                    .chunks_exact(4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::Bitfield(bitfields)
            }
            IRacingVarType::F32 => {
                let floats = bytes
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::F32(floats)
            }
            IRacingVarType::F64 => {
                let doubles = bytes
                    .chunks_exact(8)
                    .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::F64(doubles)
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
                let slice = std::slice::from_raw_parts(ptr, size::MEM_MAP_FILE_SIZE);
                Arc::from(slice)
            };

            self.shared_mem_snapshot = Some(fresh_shared_mem_snapshot.clone());
            self.header = Some(Header::new(fresh_shared_mem_snapshot));
        }

        let header = self.header.as_ref().ok_or(IRSDKError::FailedToMapView(
            "Failed to open map view".to_owned(),
        ))?;

        let mut buffers = header.var_buffers();

        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        // Take the newest one
        let mut latest = buffers
            .into_iter()
            .next()
            .ok_or(IRSDKError::InvalidSharedMemory(
                "Buffers not found".to_owned(),
            ))?;

        latest.freeze();
        self.latest_var_buffer = Some(latest);

        Ok(())
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

        let mut file = File::create(path).map_err(|e| {
            IRSDKError::UnexpectedError(eyre::eyre!("Failed to create parse to file: {e}"))
        })?;

        let memory = self.shared_mem_snapshot.as_ref().ok_or("No memory found")?;
        let header = self.header.as_ref().ok_or(IRSDKError::InvalidSharedMemory(
            "Header not found".to_owned(),
        ))?;

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

        self.shared_mem_snapshot = None;
        self.header = None;
        self.var_headers.clear();
        self.var_headers_hash.clear();
        self.var_header_names.clear();
        self.latest_var_buffer = None;
        self.test_file = None;
    }
}

impl Drop for IracingClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}
