use std::sync::Arc;

use crate::sdk::var_buffer::VarBuffer;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    shared_mem: Arc<[u8]>,
    var_buf: Vec<VarBuffer>,
}

impl Header {
    pub fn new(shared_mem: Arc<[u8]>) -> Self {
        let num_buf = Self::read_i32(&shared_mem, 32) as usize;
        let buf_len = Self::read_i32(&shared_mem, 36) as usize;

        let var_buf = (0..num_buf)
            .map(|i| {
                let offset = 48 + i * 16;
                VarBuffer::new(shared_mem.clone(), buf_len, offset)
            })
            .collect();

        Self {
            shared_mem,
            var_buf,
        }
    }

    fn read_i32(mem: &[u8], offset: usize) -> i32 {
        let bytes = &mem[offset..offset + 4];
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    // 📝 Getters for each field
    pub fn version(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 0)
    }

    pub fn status(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 4)
    }

    pub fn tick_rate(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 8)
    }

    pub fn session_info_update(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 12)
    }

    pub fn session_info_len(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 16)
    }

    pub fn session_info_offset(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 20)
    }

    pub fn num_vars(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 24)
    }

    pub fn var_header_offset(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 28)
    }

    pub fn num_buf(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 32)
    }

    pub fn buf_len(&self) -> i32 {
        Self::read_i32(&self.shared_mem, 36)
    }

    pub fn var_buffers(&self) -> &Vec<VarBuffer> {
        &self.var_buf
    }
}
