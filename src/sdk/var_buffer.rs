#[repr(C)]
#[derive(Debug, Clone)]
pub struct VarBuffer {
    pub tick_count: i32, // 0 offset
    buff_offset: i32,    // 4 offset

    frozen_memory: Option<i32>,
    pub is_memory_frozen: bool,
    buff_len: i32,
}

impl VarBuffer {
    // pub fn new(&mut self, offset: usize, shared_mem: *mut u8, buff_len: i32) -> Self {
    //     Self {
    //         tick_count: (),
    //         buff_offset: (),
    //         frozen_memory: None,
    //         is_memory_frozen: false,
    //         buff_len,
    //     }
    // }
}
