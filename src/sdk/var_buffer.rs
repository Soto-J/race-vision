#[repr(C)]
#[derive(Debug, Clone)]
pub struct VarBuffer {
    pub tick_count: i32, // 0 offset
    buff_offset: i32,    // 4 offset
}

impl VarBuffer {
    pub fn new(offset: usize, shared_mem: *mut u8, buff_len: i32) {}
}
