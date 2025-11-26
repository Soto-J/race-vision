use crate::{
    domain::iracing_errors::{ClientError, MMapError, ResolverError, SharedMemoryError},
    iracing_client::{
        broadcast::Broadcast,
        session_state::SessionState,
        telemetry::{MemoryMap, TelemetryResolver, TelemetryValue, raw::Header},
    },
    utils::constants::{
        SIM_STATUS_URL,
        size::{self},
    },
};

use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Default)]
pub struct Client {
    pub is_initialized: bool,

    pub mmap: MemoryMap,
    pub cache: TelemetryResolver,
    pub session_state: SessionState,
    pub broadcast: Option<Broadcast>,
}

impl Client {
    pub async fn init(&mut self) -> Result<(), ClientError> {
        check_sim_status().await?;

        self.mmap.load_live_data()?;

        let snap_shot = self
            .mmap
            .snapshot
            .as_ref()
            .ok_or_else(|| MMapError::SnapshotNotFound)?;

        self.cache.parse_headers(snap_shot)?;

        self.is_initialized = true;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ClientError> {
        self.init().await?;

        loop {
            self.update_latest_var_buffer()?;
            tokio::time::sleep(std::time::Duration::from_millis(16)).await;
        }
    }

    // Get all buffers and find the most recent one (highest tick_count)
    pub fn update_latest_var_buffer(&mut self) -> Result<(), ClientError> {
        let latest_var_buffer = self
            .cache
            .latest_var_buffer
            .as_mut()
            .ok_or_else(|| SharedMemoryError::BufferNotFound)?;

        latest_var_buffer.unfreeze();

        self.mmap.wait_for_valid_data_event()?;

        let view_ptr: *const u8 = self
            .mmap
            .view_ptr
            .ok_or_else(|| MMapError::ViewPtrNotFound)?;

        let fresh_snapshot: Arc<[u8]> = unsafe {
            let slice = std::slice::from_raw_parts(view_ptr, size::MEM_MAP_FILE_SIZE);
            Arc::from(slice)
        };

        /* Create a fresh Arc<[u8]> from the mapped pointer to get live data */
        self.mmap.snapshot = Some(fresh_snapshot.clone());
        self.cache.header = Some(Header::parse(fresh_snapshot)?);

        let header = self
            .cache
            .header
            .as_ref()
            .ok_or_else(|| ResolverError::HeaderNotFound)?;

        let mut buffers = header.var_buffers()?;

        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        let mut latest_buffers = buffers
            .into_iter()
            .next()
            .ok_or_else(|| SharedMemoryError::BufferNotFound)?;

        latest_buffers.freeze();

        self.cache.latest_var_buffer = Some(latest_buffers);

        Ok(())
    }

    pub fn read_value(&self, key: &str) -> Result<TelemetryValue, ClientError> {
        self.cache.get_value(key)
    }

    pub fn session_info_update(&self) -> Option<i32> {
        match &self.cache.header {
            Some(header) => Some(header.session_info_update()),
            None => None,
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.is_initialized = false;
        self.broadcast = None;
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

pub async fn check_sim_status() -> Result<(), ClientError> {
    reqwest::get(SIM_STATUS_URL).await?;
    Ok(())
}
