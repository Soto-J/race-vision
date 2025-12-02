/// Errors that occur during shared memory operations
#[derive(Debug, thiserror::Error)]
pub enum SharedMemoryError {
    #[error("out of bounds access: {0}")]
    OutOfBounds(String),

    // Buffer errors
    #[error("invalid buffer offset: {0}")]
    InvalidBufferOffset(i32),
    #[error("buffer not found")]
    BufferNotFound,

    // Calculation overflow errors
    #[error("buffer length calculation overflowed")]
    LengthOverflow,
    #[error("size calculation overflowed")]
    SizeOverflow,
    #[error("offset calculation overflowed (offset={offset}, len={len})")]
    OffsetOverflow { offset: usize, len: usize },

    // Memory mapping errors
    #[error("invalid shared memory: {0}")]
    InvalidSharedMemory(&'static str),
    #[error("failed to map memory view: {0}")]
    MapViewFailed(String),
    #[error("invalid handle: {0}")]
    InvalidHandle(String),
}
