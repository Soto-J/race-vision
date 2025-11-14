use crate::{
    client::{
        broadcast::Broadcast,
        error::IRSDKError,
        helpers::{check_sim_status, slice_var_bytes},
        session_state::SessionState,
        telemetry::{MemoryMap, VarCache, models::Header},
    },
    utils::{
        constants::size,
        enums::{IRacingVarType, VarData},
    },
};

use color_eyre::eyre::{self, ContextCompat, Ok, eyre};
use memmap2::MmapOptions;
use std::{fs::File, io::Write, path::PathBuf, sync::Arc};

#[repr(C)]
#[derive(Debug, Default)]
pub struct IracingClient {
    pub is_initialized: bool,
    pub test_file: Option<File>,
    pub parse_yaml_async: bool,

    pub mmap: MemoryMap,
    pub cache: VarCache,
    pub session_state: SessionState,
    pub broadcast: Option<Broadcast>,
}

impl IracingClient {
    pub async fn start_up(
        &mut self,
        test_file: Option<PathBuf>,
        dump_path: Option<PathBuf>,
    ) -> eyre::Result<()> {
        match test_file {
            Some(file) => {
                let file = File::open(file).map_err(|e| {
                    IRSDKError::UnexpectedError(eyre::eyre!("Failed to open test file: {}", e))
                })?;

                let mmap = unsafe { MmapOptions::new().map(&file) }.map_err(|e| {
                    IRSDKError::FailedToMapView(format!("Failed to map test file to memory: {}", e))
                })?;

                self.mmap.snapshot = Some(Arc::from(mmap.as_ref()));
            }

            None => {
                check_sim_status()
                    .await
                    .map_err(|_| IRSDKError::NotConnected)?;

                self.mmap.load_live_data()?;

                let snap_shot = self
                    .mmap
                    .snapshot
                    .as_ref()
                    .ok_or_else(|| eyre!(IRSDKError::SnapshotNotFound))?;

                self.cache.parse_headers(snap_shot)?;
            }
        };

        if let Some(path) = dump_path {
            let snapshot = self
                .mmap
                .snapshot
                .as_ref()
                .ok_or_else(|| eyre!(IRSDKError::SnapshotNotFound))?;

            let mut file = File::create(path).map_err(|_| {
                IRSDKError::UnexpectedError(eyre::eyre!("Failed to create dump file."))
            })?;

            file.write_all(snapshot).map_err(|_| {
                IRSDKError::UnexpectedError(eyre::eyre!("Failed to write to dump file"))
            })?;
        }

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
        if let Some(var_buffer) = &mut self.cache.latest_var_buffer {
            var_buffer.unfreeze();
        }
    }

    // Get all buffers and find the most recent one (highest tick_count)
    pub fn freeze_latest_var_buffer(&mut self) -> eyre::Result<()> {
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

    pub fn parse_to(&self, path: String) -> eyre::Result<()> {
        if !self.is_initialized {
            return Err(eyre!(""));
        }

        let mut file = File::create(path).map_err(|e| {
            IRSDKError::UnexpectedError(eyre::eyre!("Failed to create parse to file: {e}"))
        })?;

        let memory = self
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| IRSDKError::SnapshotNotFound)?;

        let header = self
            .cache
            .header
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory(
                "Header not found".to_owned(),
            ))?;

        let offset = header.session_info_offset() as usize;
        let len = header.session_info_len() as usize;

        /* Write session info YAML */
        file.write_all(&memory[offset..offset + len])?;
        file.write_all(b"\n")?;

        /* Write variable headers */
        for (key, var_header) in &self.cache.var_headers_hash {
            let line = format!("{:32}{}\n", key, var_header);

            file.write_all(line.as_bytes())?;
        }

        Ok(())
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
