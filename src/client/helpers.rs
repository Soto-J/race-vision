use crate::{
    client::error::IRSDKError,
    utils::{
        constants::{SIM_STATUS_URL, size},
        enums::IRacingVarType,
    },
};

#[cfg(windows)]
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Memory::{FILE_MAP_READ, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile},
};

pub async fn check_sim_status() -> Result<(), reqwest::Error> {
    let response = reqwest::get(SIM_STATUS_URL).await?;
    println!("Sim Status: {:?}", response.status());
    Ok(())
}

pub fn map_to_address(
    mem_handle: HANDLE,
) -> Result<(*const u8, MEMORY_MAPPED_VIEW_ADDRESS), IRSDKError> {
    // Map it into our address space
    let address_space = unsafe {
        MapViewOfFile(
            mem_handle.clone(),
            FILE_MAP_READ,
            0,
            0,
            size::MEM_MAP_FILE_SIZE,
        )
    };

    let mapping_view_ptr = address_space.Value as *const u8;

    if mapping_view_ptr.is_null() {
        let _ = unsafe { CloseHandle(mem_handle.clone()) };
        return Err(IRSDKError::FailedToMapView(
            "Map view of file returned null pointer",
        ));
    }

    Ok((mapping_view_ptr, address_space))
}

pub fn slice_var_bytes<'a>(
    memory: &'a [u8],
    offset: usize,
    count: usize,
    var_type: i32,
) -> Result<(&'a [u8], IRacingVarType), IRSDKError> {
    let var_type =
        IRacingVarType::try_from(var_type).map_err(|_| IRSDKError::InvalidVarType(var_type))?;

    // Calculate the total size needed for bounds checking
    let size_per_element = match var_type {
        IRacingVarType::Char8 | IRacingVarType::Bool => 1,
        IRacingVarType::I32 | IRacingVarType::Bitfield | IRacingVarType::F32 => 4,
        IRacingVarType::F64 => 8,
    };

    let byte_len = count
        .checked_mul(size_per_element)
        .ok_or_else(|| IRSDKError::InvalidSharedMemory("Size calculation overflowed"))?;

    let end_offset = offset
        .checked_add(byte_len)
        .ok_or_else(|| IRSDKError::InvalidSharedMemory("Offset calculation overflowed"))?;

    if end_offset > memory.len() {
        return Err(IRSDKError::InvalidSharedMemory(
            "Variable data range exceeds buffer size",
        ));
    }

    Ok((&memory[offset..end_offset], var_type))
}
