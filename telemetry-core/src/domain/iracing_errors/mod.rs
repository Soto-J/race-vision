pub mod mmap_error;
pub mod resolver_error;
pub mod shared_memory_error;

pub use color_eyre::eyre;
pub use mmap_error::*;
pub use resolver_error::*;
pub use shared_memory_error::*;

// Top-level error type for the iRacing SDK
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Resolver(#[from] ResolverError),
    #[error(transparent)]
    SharedMemory(#[from] SharedMemoryError),
    #[error(transparent)]
    MMap(#[from] MMapError),

    #[error(transparent)]
    Parser(#[from] ParseError),

    #[error("iRacing is not running or shared memory is not available")]
    NotConnected,

    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("operation timed out")]
    Timeout,

    #[error("unexpected error")]
    UnexpectedError(#[source] eyre::Report),
}

impl From<eyre::Report> for ClientError {
    fn from(e: eyre::Report) -> Self {
        ClientError::UnexpectedError(e)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("failed to convert chunk to array: {0}")]
    InvalidChunk(String),

    #[error("unknown variable type: {0}")]
    UnknownVarKind(i32),
}
