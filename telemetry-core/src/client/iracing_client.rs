use crate::{
    check_sim_status,
    client::{
        broadcast::Broadcast,
        error::IRSDKError,
        session_state::SessionState,
        telemetry::{MemoryMap, VarCache, models::Header},
    },
    utils::{
        constants::size::{self},
        enums::var_types::TelemetryValue,
    },
};

use color_eyre::eyre::{self, ContextCompat, Ok, eyre};
use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Default)]
pub struct IracingClient {
    pub is_initialized: bool,

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

    pub fn get_item(&self, key: &str) -> eyre::Result<TelemetryValue> {
        self.cache.get_value(key)
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

    pub fn shut_down(&mut self) {
        self.is_initialized = false;

        self.broadcast = None;
    }
}

impl Drop for IracingClient {
    fn drop(&mut self) {
        self.shut_down();
    }
}
