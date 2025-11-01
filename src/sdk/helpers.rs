use crate::{
    sdk::error::IRSDKError,
    utils::constants::{
        DATA_VALID_EVENT_NAME, MEM_MAP_FILE, MEM_MAP_FILE_SIZE, SIM_STATUS_URL, SYNCHRONIZE_ACCESS,
    },
};

use std::{
    error,
    ffi::{CString, OsStr},
    fs::File,
    io::Write,
    os::windows::ffi::OsStrExt,
    path::Path,
    sync::Arc,
};

use windows::Win32::System::Threading::{OpenEventW, SYNCHRONIZATION_ACCESS_RIGHTS};
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Memory::{
            FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile, OpenFileMappingA,
        },
    },
    core::{PCSTR, PCWSTR},
};

pub async fn check_sim_status() -> Result<(), reqwest::Error> {
    let response = reqwest::get(SIM_STATUS_URL).await?;
    println!("Sim Status: {:?}", response.status());
    Ok(())
}

pub fn open_memory_mapped_file() -> Result<HANDLE, Box<dyn error::Error>> {
    // Create a null-terminated string for the Windows API
    let mem_map_name = CString::new(MEM_MAP_FILE)?;

    let handle = unsafe {
        OpenFileMappingA(
            FILE_MAP_READ.0,
            false,
            PCSTR(mem_map_name.as_ptr() as *const u8),
        )
    }?;

    Ok(handle)
}

pub fn map_to_address(
    mem_handle: HANDLE,
) -> Result<(*const u8, MEMORY_MAPPED_VIEW_ADDRESS), IRSDKError> {
    // Map it into our address space
    let address_space =
        unsafe { MapViewOfFile(mem_handle.clone(), FILE_MAP_READ, 0, 0, MEM_MAP_FILE_SIZE) };

    let base_ptr = address_space.Value as *const u8;

    if base_ptr.is_null() {
        let _ = unsafe { CloseHandle(mem_handle.clone()) };
        return Err(IRSDKError::FailedToMapView(
            "Map view of file returned null pointer",
        ));
    }

    // Return the raw pointer - we'll create Arc<[u8]> on-demand for live reads
    Ok((base_ptr, address_space))
}
