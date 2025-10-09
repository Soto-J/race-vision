pub struct VarBuffer {
    pub tick_count: i32, // 0 offset
    buff_offset: i32,    // 4 offset
}

impl VarBuffer {
    pub fn new(shared_mem: *mut u8, buff_len: i32) {}
}
