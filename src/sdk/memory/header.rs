use std::{fmt, sync::Arc};

use crate::sdk::VarBuffer;

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
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn version(&self) -> i32 {
        self.read_i32(0)
    }

    pub fn status(&self) -> i32 {
        self.read_i32(4)
    }

    pub fn tick_rate(&self) -> i32 {
        self.read_i32(8)
    }

    pub fn session_info_update(&self) -> i32 {
        self.read_i32(12)
    }

    pub fn session_info_len(&self) -> i32 {
        self.read_i32(16)
    }

    pub fn session_info_offset(&self) -> i32 {
        self.read_i32(20)
    }

    pub fn num_vars(&self) -> i32 {
        self.read_i32(24)
    }

    pub fn var_header_offset(&self) -> i32 {
        self.read_i32(28)
    }

    pub fn num_buf(&self) -> i32 {
        self.read_i32(32)
    }

    pub fn buf_len(&self) -> i32 {
        self.read_i32(36)
    }

    pub fn var_buffers(&self) -> Vec<VarBuffer> {
        let num_buf = self.num_buf().max(0) as usize;
        let buf_len = self.buf_len().max(0) as usize;
        let mut buffers = Vec::with_capacity(num_buf);

        for i in 0..num_buf {
            let offset = 48 + i * 16;
            let vb = VarBuffer::new(self.shared_mem.clone(), buf_len, offset);
            buffers.push(vb);
        }

        buffers
    }
}
