use crate::{
    domain::iracing_errors::{ClientError, ResolverError},
    iracing_client::telemetry::VarBuffer,
    utils::constants::size::{ByteSize, HEADER_OFFSET, VAR_BUF_OFFSET},
};

use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    shared_mem: Arc<[u8]>,
}

impl Header {
    pub fn parse(shared_mem: Arc<[u8]>) -> Result<Self, ClientError> {
        let header = Self { shared_mem };
        header.validate()?;

        Ok(header)
    }

    pub fn var_buffers(&self) -> Result<Vec<VarBuffer>, ClientError> {
        let buf_len = self.buf_len() as usize;

        (0..self.num_buf() as usize)
            .map(|i| {
                let offset = HEADER_OFFSET + (VAR_BUF_OFFSET * i);

                VarBuffer::parse(self.shared_mem.clone(), buf_len, offset)
            })
            .collect()
    }

    pub fn version(&self) -> i32 {
        self.read_i32(0 * ByteSize::I32)
    }

    pub fn status(&self) -> i32 {
        self.read_i32(1 * ByteSize::I32)
    }

    pub fn tick_rate(&self) -> i32 {
        self.read_i32(2 * ByteSize::I32)
    }

    pub fn session_info_update(&self) -> i32 {
        self.read_i32(3 * ByteSize::I32)
    }

    pub fn session_info_len(&self) -> i32 {
        self.read_i32(4 * ByteSize::I32)
    }

    pub fn session_info_offset(&self) -> i32 {
        self.read_i32(5 * ByteSize::I32)
    }

    pub fn num_vars(&self) -> i32 {
        self.read_i32(6 * ByteSize::I32)
    }

    pub fn var_header_offset(&self) -> i32 {
        self.read_i32(7 * ByteSize::I32)
    }

    pub fn num_buf(&self) -> i32 {
        self.read_i32(8 * ByteSize::I32)
    }

    pub fn buf_len(&self) -> i32 {
        self.read_i32(9 * ByteSize::I32)
    }

    fn read_i32(&self, offset: usize) -> i32 {
        let bytes = &self.shared_mem[offset..offset + ByteSize::I32];
        i32::from_le_bytes(
            bytes
                .try_into()
                .expect("Failed to convert bytes to little endian i32"),
        )
    }

    fn validate(&self) -> Result<(), ClientError> {
        // Check minimum buffer size
        if self.shared_mem.len() < HEADER_OFFSET {
            return Err(ResolverError::HeaderBufferTooSmall {
                expected: HEADER_OFFSET,
                actual: self.shared_mem.len(),
            }
            .into());
        }

        // Validate version
        let version = self.version();
        if version < 1 || version > 3 {
            return Err(ResolverError::UnsupportedVersion(version).into());
        }

        // Validate num_vars
        let num_vars = self.num_vars();
        if num_vars < 0 {
            return Err(ResolverError::InvalidHeaderField {
                field: "num_vars".to_string(),
                reason: format!("cannot be negative: {}", num_vars),
            }
            .into());
        }
        if num_vars > 10_000 {
            // Reasonable upper limit
            return Err(ResolverError::InvalidHeaderField {
                field: "num_vars".to_string(),
                reason: format!("exceeds reasonable maximum: {}", num_vars),
            }
            .into());
        }

        // 4. Validate num_buf
        let num_buf = self.num_buf();
        if num_buf < 0 {
            return Err(ResolverError::InvalidHeaderField {
                field: "num_buf".to_string(),
                reason: format!("cannot be negative: {}", num_buf),
            }
            .into());
        }
        if num_buf > 100 {
            // iRacing typically uses 3-4 buffers
            return Err(ResolverError::InvalidHeaderField {
                field: "num_buf".to_string(),
                reason: format!("exceeds reasonable maximum: {}", num_buf),
            }
            .into());
        }

        // 5. Validate buf_len
        let buf_len = self.buf_len();
        if buf_len < 0 {
            return Err(ResolverError::InvalidHeaderField {
                field: "buf_len".to_string(),
                reason: format!("cannot be negative: {}", buf_len),
            }
            .into());
        }

        // 6. Validate tick_rate
        let tick_rate = self.tick_rate();
        if tick_rate <= 0 {
            return Err(ResolverError::InvalidHeaderField {
                field: "tick_rate".to_string(),
                reason: format!("must be positive: {}", tick_rate),
            }
            .into());
        }

        // 7. Validate var_header_offset
        let var_header_offset = self.var_header_offset();
        if var_header_offset < 0 || var_header_offset as usize >= self.shared_mem.len() {
            return Err(ResolverError::InvalidHeaderField {
                field: "var_header_offset".to_string(),
                reason: format!("offset {} is outside buffer bounds", var_header_offset),
            }
            .into());
        }

        // 8. Validate session_info_offset if present
        let session_info_len = self.session_info_len();
        if session_info_len > 0 {
            let session_info_offset = self.session_info_offset();

            if session_info_offset < 0 || session_info_offset as usize >= self.shared_mem.len() {
                return Err(ResolverError::InvalidHeaderField {
                    field: "session_info_offset".to_string(),
                    reason: format!("offset {} is outside buffer bounds", session_info_offset),
                }
                .into());
            }
        }

        Ok(())
    }
}
