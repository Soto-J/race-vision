pub const MEM_MAP_FILE_SIZE: usize = 1164 * 1024;

// Header
pub const HEADER_OFFSET: usize = 10 * ByteSize::I32 + ByteSize::PADDING;
pub const HEADER_PADDING: usize = ByteSize::I64;

// Var Header
pub const VAR_BUF_OFFSET: usize = 4 * ByteSize::I32;
pub const VAR_HEADER_SIZE: usize = 144;
pub const MAX_REASONABLE_VARS: i32 = 10_000;

pub const KIND_START: usize = 0;
pub const KIND_END: usize = 4;

pub const OFFSET_START: usize = 4;
pub const OFFSET_END: usize = 8;

pub const COUNT_START: usize = 8;
pub const COUNT_END: usize = 12;

pub const COUNT_AS_TIME_BYTE: usize = 12;

pub const NAME_START: usize = 16;
pub const NAME_END: usize = 48;

pub const DESCRIPTION_START: usize = 48;
pub const DESCRIPTION_END: usize = 112;

pub const UNIT_START: usize = 112;
pub const UNIT_END: usize = 144;

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
