use color_eyre::eyre::{self, Ok, eyre};

#[cfg(windows)]
use std::{ffi, os::windows::ffi::OsStrExt, sync::Arc};
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0},
        System::{
            Memory::{self, FILE_MAP_READ, MapViewOfFile},
            Threading,
        },
    },
    core::{PCSTR, PCWSTR},
};

use crate::{
    client::error::IRSDKError,
    utils::constants::{self, size},
};

#[repr(C)]
#[derive(Debug, Default)]
pub struct MemoryMap {
    #[cfg(windows)]
    pub snapshot: Option<Arc<[u8]>>,
    #[cfg(windows)]
    pub file_handle: Option<HANDLE>,
    #[cfg(windows)]
    pub view_ptr: Option<*const u8>,
    #[cfg(windows)]
    pub view_addr: Option<*mut ffi::c_void>,
    #[cfg(windows)]
    pub data_valid_event: Option<HANDLE>,
}

impl MemoryMap {
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
    pub fn load_live_data(&mut self) -> eyre::Result<()> {
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
        .map_err(|e| eyre!(IRSDKError::InvalidHandle(e.to_string())))?;

        self.data_valid_event = Some(handle);

        self.wait_for_valid_data_event()?;
        self.open_memory_map()?;

        Ok(())
    }

    pub fn load_test_file() {}

    pub fn wait_for_valid_data_event(&self) -> eyre::Result<()> {
        let handle = self
            .data_valid_event
            .ok_or_else(|| IRSDKError::DataValidEventNotFound)?;

        unsafe {
            let wait_result = Threading::WaitForSingleObject(handle, 32);

            if wait_result == WAIT_OBJECT_0 {
                Ok(())
            } else {
                Err(eyre::eyre!(IRSDKError::Timeout))
            }
        }
    }

    // opens a handle to a memory-mapped file and maps it into the process's address space
    pub fn open_memory_map(&mut self) -> eyre::Result<()> {
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

            return Err(eyre::eyre!(IRSDKError::FailedToMapView(
                "Map view of file returned null pointer".to_owned(),
            )));
        }

        // Store the raw pointer and address space
        self.file_handle = Some(handle);
        self.view_ptr = Some(mapping_view_ptr);
        self.view_addr = Some(memory_map_view.Value as *mut ffi::c_void);

        // Create initial snapshot from pointer
        let shared_ptr: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(mapping_view_ptr, size::MEM_MAP_FILE_SIZE);
            Arc::from(slice)
        };

        self.snapshot = Some(shared_ptr);

        Ok(())
    }

    fn shutdown(&mut self) {
        /* Close OS handles safely */
        /* Take guarantees the field becomes None*/
        unsafe {
            if let Some(handle) = self.data_valid_event.take() {
                let _ = CloseHandle(handle);
            }

            if let Some(ptr) = self.view_addr.take() {
                let addr = Memory::MEMORY_MAPPED_VIEW_ADDRESS { Value: ptr };
                let _ = Memory::UnmapViewOfFile(addr);
            }

            if let Some(handle) = self.file_handle.take() {
                let _ = CloseHandle(handle);
            }
        }
    }
}

impl Drop for MemoryMap {
    fn drop(&mut self) {
        self.shutdown();
    }
}
