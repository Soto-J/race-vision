use crate::sdk::memory::var_buffer::VarBuffer;

use std::{fmt, sync::Arc};

const I32_SIZE: usize = (i32::BITS / 8) as usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    shared_mem: Arc<[u8]>,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header: {{ version: {}, status: {}, tick_rate: {}, num_vars: {}, num_buf: {}, buf_len: {} }}",
            self.version(),
            self.status(),
            self.tick_rate(),
            self.num_vars(),
            self.num_buf(),
            self.buf_len()
        )
    }
}

impl Header {
    pub fn new(shared_mem: Arc<[u8]>) -> Self {
        let header = Self { shared_mem };
        println!("{}", header);
        header
    }

    fn read_i32(&self, offset: usize) -> i32 {
        let bytes = &self.shared_mem[offset..offset + 4];
        i32::from_le_bytes(
            bytes
                .try_into()
                .expect("Failed to convert bytes to little endian i32"),
        )
    }

    pub fn version(&self) -> i32 {
        self.read_i32(0 * I32_SIZE)
    }

    pub fn status(&self) -> i32 {
        self.read_i32(4 * I32_SIZE)
    }

    pub fn tick_rate(&self) -> i32 {
        self.read_i32(8 * I32_SIZE)
    }

    pub fn session_info_update(&self) -> i32 {
        self.read_i32(12 * I32_SIZE)
    }

    pub fn session_info_len(&self) -> i32 {
        self.read_i32(16 * I32_SIZE)
    }

    pub fn session_info_offset(&self) -> i32 {
        self.read_i32(20 * I32_SIZE)
    }

    pub fn num_vars(&self) -> i32 {
        self.read_i32(24 * I32_SIZE)
    }

    pub fn var_header_offset(&self) -> i32 {
        self.read_i32(28 * I32_SIZE)
    }

    pub fn num_buf(&self) -> i32 {
        self.read_i32(32 * I32_SIZE)
    }

    pub fn buf_len(&self) -> i32 {
        self.read_i32(36 * I32_SIZE)
    }

    pub fn var_buffers(&self) -> Vec<VarBuffer> {
        let num_buf = self.num_buf().max(0) as usize;
        let buf_len = self.buf_len().max(0) as usize;

        (0..num_buf)
            .map(|i| {
                let offset = 48 + i * 16;

                VarBuffer::new(self.shared_mem.clone(), buf_len, offset)
            })
            .collect()
    }
}
