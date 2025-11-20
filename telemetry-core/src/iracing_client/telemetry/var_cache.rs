use color_eyre::eyre::{self, Ok, eyre};

use crate::{
    iracing_client::{
        error::IRSDKError,
        telemetry::{
            TelemetryValue, VarBuffer, VarKind, raw::{Header, VarHeader}
        },
    },
    utils::constants::size,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Default)]
pub struct VarCache {
    pub header: Option<Header>,
    pub var_headers: Vec<VarHeader>,
    pub var_headers_hash: HashMap<String, VarHeader>,
    pub latest_var_buffer: Option<VarBuffer>,
}

impl VarCache {
    pub fn get_value(&self, key: &str) -> eyre::Result<TelemetryValue> {
        let var_header = self
            .var_headers_hash
            .get(key)
            .ok_or(IRSDKError::VarHeaderNotFound)?;

        let var_kind = VarKind::try_from(var_header.var_type)
            .map_err(|e| IRSDKError::UnexpectedError(eyre!(e)))?;

        let bytes_per_element = match var_kind {
            VarKind::Char8 | VarKind::Bool => 1,
            VarKind::I32 | VarKind::Bitfield | VarKind::F32 => 4,
            VarKind::F64 => 8,
        };

        let count = var_header.count as usize;

        let byte_len =
            count
                .checked_mul(bytes_per_element)
                .ok_or(IRSDKError::InvalidSharedMemory(
                    "Size calculation overflowed".to_owned(),
                ))?;

        let offset = var_header.offset as usize;

        let end_offset = offset
            .checked_add(byte_len)
            .ok_or(IRSDKError::InvalidSharedMemory(
                "Offset calculation overflowed".to_owned(),
            ))?;

        let latest_buffer = self
            .latest_var_buffer
            .as_ref()
            .ok_or(IRSDKError::InvalidSharedMemory("Buffer not found".into()))?;

        let snapshot = latest_buffer.get_memory();

        if end_offset > snapshot.len() {
            return Err(eyre!(IRSDKError::InvalidSharedMemory(format!(
                "Variable data range exceeds buffer size: End Offset {end_offset}, Snapshot lenagth: {}",
                snapshot.len()
            ),)));
        }

        var_kind.parse_to_value(&snapshot[offset..end_offset])
    }

    pub fn parse_headers(&mut self, memory_snapshot: &Arc<[u8]>) -> eyre::Result<()> {
        let header = Header::new(memory_snapshot.clone());

        // update to latest buffer
        let mut buffers = header.var_buffers();
        buffers.sort_by(|a, b| b.tick_count().cmp(&a.tick_count()));

        self.latest_var_buffer = buffers.get(1).cloned().or_else(|| buffers.get(0).cloned());

        self.load_var_headers(
            memory_snapshot,
            header.num_vars().max(0) as usize,
            header.var_header_offset().max(0) as usize,
        )?;

        self.header = Some(header);

        Ok(())
    }

    fn load_var_headers(
        &mut self,
        memory_snapshot: &Arc<[u8]>,
        num_vars: usize,
        base_offset: usize,
    ) -> eyre::Result<()> {
        for i in 0..num_vars {
            let offset = base_offset + i * size::VAR_HEADER_SIZE;
            let end = offset + size::VAR_HEADER_SIZE;

            if end > memory_snapshot.len() {
                break;
            }

            let var_header =
                VarHeader::from_bytes(&memory_snapshot[offset..end]).ok_or_else(|| {
                    eyre::eyre!(IRSDKError::InvalidVarHeader(
                        "Failed to create var header".to_owned()
                    ),)
                })?;

            let var_name = var_header.name_str().ok_or_else(|| {
                eyre!(IRSDKError::InvalidVarHeader(
                    "Failed to get var header name".to_owned(),
                ))
            })?;

            self.var_headers_hash
                .insert(var_name.to_string(), var_header.clone());

            self.var_headers.push(var_header);
        }

        Ok(())
    }
}
