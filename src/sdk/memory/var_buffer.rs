#![allow(unused)]

use std::sync::Arc;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarBuffer {
    shared_mem: Arc<[u8]>,
    buf_len: usize,
    offset: usize,
    frozen_memory: Option<Vec<u8>>,
    is_memory_frozen: bool,
}

impl VarBuffer {
    pub fn new(shared_mem: Arc<[u8]>, buf_len: usize, offset: usize) -> Self {
        Self {
            shared_mem,
            buf_len,
            offset,
            frozen_memory: None,
            is_memory_frozen: false,
        }
    }

    fn read_i32(&self, rel_offset: usize) -> i32 {
        let abs = self.offset + rel_offset;
        let bytes = &self.get_memory()[abs..abs + 4];
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn tick_count(&self) -> i32 {
        self.read_i32(0)
    }

    // Read `_buf_offset` at offset 4 (int32)
    pub fn buff_offset_raw(&self) -> i32 {
        let bytes = &self.get_memory()[4..8];
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn buff_offset(&self) -> i32 {
        if self.is_memory_frozen {
            return 0;
        }

        self.buff_offset_raw()
    }

    pub fn freeze(&mut self) {
        let buff_offset = self.buff_offset_raw() as usize;
        let end = buff_offset + self.buf_len;

        let frozen = self.shared_mem[buff_offset..end].to_vec();
        self.frozen_memory = Some(frozen);
        self.is_memory_frozen = true;
    }

    // Read `_buf_offset` at offset 4 (int32)
    pub fn unfreeze(&mut self) {
        self.frozen_memory = None;
        self.is_memory_frozen = false;
    }

    pub fn get_memory(&self) -> &[u8] {
        if self.is_memory_frozen {
            return self
                .frozen_memory
                .as_ref()
                .expect("frozen_memory should exist when frozen.");
        }

        &self.shared_mem
    }
}
