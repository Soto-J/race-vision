use crate::{
    client::{
        broadcast::Broadcast,
        error::IRSDKError,
        helpers::{check_sim_status, slice_var_bytes},
        session_state::SessionState,
        telemetry::{MemoryMap, VarCache, models::Header},
    },
    utils::{
        constants::size::{self, ByteSize},
        enums::{IRacingVarType, VarData},
    },
};

use color_eyre::eyre::{self, ContextCompat, Ok, eyre};
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Default)]
pub struct IracingClient {
    pub is_initialized: bool,
    pub parse_yaml_async: bool,

    pub mmap: MemoryMap,
    pub cache: VarCache,
    pub session_state: SessionState,
    pub broadcast: Option<Broadcast>,
}

impl IracingClient {
    pub async fn start_up(&mut self) -> eyre::Result<()> {
        check_sim_status().await?;

        self.mmap.load_live_data()?;

        let snap_shot = self
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| eyre!(IRSDKError::SnapshotNotFound))?;

        self.cache.parse_headers(snap_shot)?;

        self.is_initialized = true;

        Ok(())
    }

    pub fn get_item(&self, key: &str) -> eyre::Result<VarData> {
        let var_header = self
            .cache
            .var_headers_hash
            .get(key)
            .ok_or(IRSDKError::ItemNotFound)?;

        let latest_buffer =
            self.cache
                .latest_var_buffer
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
                    .chunks_exact(ByteSize::I32)
                    .map(|b| i32::from_le_bytes(b.try_into().unwrap())) /* Unwrap is safe here Since chunks_exact(4) guarantees the size*/
                    .collect();

                VarData::I32(int)
            }
            IRacingVarType::Bitfield => {
                let bitfields = bytes
                    .chunks_exact(ByteSize::I32)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::Bitfield(bitfields)
            }
            IRacingVarType::F32 => {
                let floats = bytes
                    .chunks_exact(ByteSize::F32)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::F32(floats)
            }
            IRacingVarType::F64 => {
                let doubles = bytes
                    .chunks_exact(ByteSize::F64)
                    .map(|b| f64::from_le_bytes(b.try_into().unwrap()))
                    .collect();

                VarData::F64(doubles)
            }
        };

        Ok(value)
    }

    // Get all buffers and find the most recent one (highest tick_count)
    pub fn update_latest_var_buffer(&mut self) -> eyre::Result<()> {
        let latest_var_buffer = self
            .cache
            .latest_var_buffer
            .as_mut()
            .ok_or_else(|| eyre!("Var Buffer not found"))?;

        latest_var_buffer.unfreeze();

        self.mmap.wait_for_valid_data_event()?;

        let view_ptr: *const u8 = self
            .mmap
            .view_ptr
            .ok_or_else(|| eyre!(IRSDKError::ViewPTRNotFound))?;

        let fresh_snapshot: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(view_ptr, size::MEM_MAP_FILE_SIZE);
            Arc::from(slice)
        };

        /* Create a fresh Arc<[u8]> from the mapped pointer to get live data */
        self.mmap.snapshot = Some(fresh_snapshot.clone());
        self.cache.header = Some(Header::new(fresh_snapshot));

        let header = self
            .cache
            .header
            .as_ref()
            .ok_or_else(|| eyre!(IRSDKError::HeaderNotFound))?;

        let mut buffers = header.var_buffers();

        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        let mut latest = buffers
            .into_iter()
            .next()
            .wrap_err(IRSDKError::UnexpectedError(eyre!(
                "Failed to retrieve latest var buffer"
            )))?;

        latest.freeze();

        self.cache.latest_var_buffer = Some(latest);

        Ok(())
    }

    pub fn session_info_update(&self) -> Option<i32> {
        match &self.cache.header {
            Some(header) => Some(header.session_info_update()),
            None => None,
        }
    }

    pub fn shutdown(&mut self) {
        self.is_initialized = false;

        self.broadcast = None;
    }
}

impl Drop for IracingClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}
