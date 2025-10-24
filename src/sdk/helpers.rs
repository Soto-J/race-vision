use crate::utils::{
    DATA_VALID_EVENT_NAME, MEM_MAP_FILE, MEM_MAP_FILE_SIZE, SIM_STATUS_URL, SYNCHRONIZE_ACCESS,
};

use std::{
    error,
    ffi::{CString, OsStr},
    fs::File,
    os::windows::ffi::OsStrExt,
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

pub fn open_data_valid_event() -> Result<HANDLE, Box<dyn error::Error>> {
    // Convert String to Wide String (UTF-16)
    let wide_name: Vec<u16> = OsStr::new(DATA_VALID_EVENT_NAME)
        .encode_wide()
        .chain(Some(0))
        .collect();

    let handle = unsafe {
        OpenEventW(
            SYNCHRONIZATION_ACCESS_RIGHTS(SYNCHRONIZE_ACCESS), // This requests permission to wait on the event.
            false, // Don’t inherit this handle by child processes
            PCWSTR(wide_name.as_ptr()),
        )
    }?;

    Ok(handle)
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

pub fn open_test_file_mmap(file_path: &str) -> Result<Arc<[u8]>, Box<dyn error::Error>> {
    use memmap2::MmapOptions;

    let file = File::open(file_path)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // Convert mmap to Arc<[u8]> to match the shared memory format
    let data: Arc<[u8]> = Arc::from(mmap.as_ref());

    Ok(data)
}

pub fn map_to_address(
    mem_handle: HANDLE,
) -> Result<(Arc<[u8]>, MEMORY_MAPPED_VIEW_ADDRESS), Box<dyn error::Error>> {
    // Map it into our address space
    let address_space =
        unsafe { MapViewOfFile(mem_handle.clone(), FILE_MAP_READ, 0, 0, MEM_MAP_FILE_SIZE) };

    let base_ptr = address_space.Value as *const u8;

    if base_ptr.is_null() {
        let _ = unsafe { CloseHandle(mem_handle.clone()) };
        return Err("MapViewOfFile returned null pointer".into());
    }

    let shared_ptr: Arc<[u8]> = unsafe {
        let slice = std::slice::from_raw_parts(base_ptr, MEM_MAP_FILE_SIZE);
        Arc::from(slice.to_vec())
    };

    Ok((shared_ptr, address_space))
}
