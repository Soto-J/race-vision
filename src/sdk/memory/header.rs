use crate::{sdk::memory::var_buffer::VarBuffer, utils::constants::size::ByteSize};

use std::{fmt, sync::Arc};

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
