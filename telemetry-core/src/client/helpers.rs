use crate::{
    client::error::IRSDKError,
    utils::{constants::SIM_STATUS_URL, enums::IRacingVarType},
};

use color_eyre::eyre::{self, Ok, eyre};

pub async fn check_sim_status() -> eyre::Result<()> {
    let res = reqwest::get(SIM_STATUS_URL).await?;
    println!("Sim Status: {:?}", res.status());
    Ok(())
}

pub fn slice_var_bytes<'a>(
    memory: &'a [u8],
    offset: usize,
    count: usize,
    var_type: i32,
) -> eyre::Result<(&'a [u8], IRacingVarType)> {
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
        .ok_or(IRSDKError::InvalidSharedMemory(
            "Size calculation overflowed".to_owned(),
        ))?;

    let end_offset = offset
        .checked_add(byte_len)
        .ok_or(IRSDKError::InvalidSharedMemory(
            "Offset calculation overflowed".to_owned(),
        ))?;

    if end_offset > memory.len() {
        return Err(eyre!(IRSDKError::InvalidSharedMemory(
            "Variable data range exceeds buffer size".to_owned(),
        )));
    }

    Ok((&memory[offset..end_offset], var_type))
}
