pub const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;

pub const HEADER_PADDING: usize = ByteSize::I64;
pub const VAR_BUF_SIZE: usize = 4 * ByteSize::I32;
pub const VAR_HEADER_SIZE: usize = 144;

pub struct ByteSize;
impl ByteSize {
    pub const PADDING: usize = 8;

    pub const I8: usize = std::mem::size_of::<i8>();
    pub const U8: usize = std::mem::size_of::<u8>();
    pub const I16: usize = std::mem::size_of::<i16>();
    pub const U16: usize = std::mem::size_of::<u16>();
    pub const I32: usize = std::mem::size_of::<i32>();
    pub const U32: usize = std::mem::size_of::<u32>();
    pub const I64: usize = std::mem::size_of::<i64>();
    pub const U64: usize = std::mem::size_of::<u64>();
    pub const F32: usize = std::mem::size_of::<f32>();
    pub const F64: usize = std::mem::size_of::<f64>();
}
