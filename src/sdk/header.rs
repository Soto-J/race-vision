use crate::sdk::var_buffer::VarBuffer;

#[repr(C)]
pub struct Header {
    pub version: i32,   // offset: 0
    pub status: i32,    // offset: 4
    pub tick_rate: i32, // offset: 8

    pub session_info_update: i32, // offset: 12
    pub session_info_len: i32,    // offset: 16
    pub session_info_offset: i32, // offset: 20

    pub num_vars: i32,          // offset: 24
    pub var_header_offset: i32, // offset: 28

    pub num_buf: i32, // offset: 32
    pub buf_len: i32, // offset: 36
}

impl Header {
    pub fn create_var_buffers(self, shared_mem: *mut u8) -> Vec<VarBuffer> {
        (0..self.num_buf)
            .map(|i| VarBuffer::new(shared_mem, 48 + i * 16))
            .collect()
    }
}
