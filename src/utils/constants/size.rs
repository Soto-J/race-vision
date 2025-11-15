pub const VAR_HEADER_SIZE: usize = 144;
pub const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;

pub struct ByteSize;
impl ByteSize {
    pub const I8: usize = std::mem::size_of::<i8>();
    pub const U8: usize = std::mem::size_of::<u8>();
    pub const I32: usize = std::mem::size_of::<i32>();
    pub const U32: usize = std::mem::size_of::<u32>();
    pub const F32: usize = std::mem::size_of::<f32>();
    pub const F64: usize = std::mem::size_of::<f64>();
}
