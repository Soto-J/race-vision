/// Errors that occur during shared memory operations
#[derive(Debug, thiserror::Error)]
pub enum SharedMemoryError {
    // Memory access errors
    #[error("integer read out of bounds at offset {offset}")]
    ReadOutOfBounds { offset: usize },
    #[error("slice out of bounds: start={start}, end={end}, mem_len={mem_len}")]
    SliceOutOfBounds {
        start: usize,
        end: usize,
        mem_len: usize,
    },
    #[error(
        "variable data range exceeds buffer bounds: end={end_offset}, snapshot_len={snapshot_len}"
    )]
    OutOfBounds {
        end_offset: usize,
        snapshot_len: usize,
    },

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
