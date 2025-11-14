use color_eyre::eyre::{self, eyre};

use crate::{
    client::{
        error::IRSDKError,
        telemetry::{
            VarBuffer,
            models::{Header, VarHeader},
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
