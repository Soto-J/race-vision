/// Errors that occur during parsing and validation of telemetry data
#[derive(Debug, thiserror::Error)]
pub enum ResolverError {
    #[error("unsupported telemetry version: {0}")]
    UnsupportedVersion(i32),

    // Header errors
    #[error("header not found")]
    HeaderNotFound,
    #[error("invalid header field '{field}': {reason}")]
    InvalidHeaderField { field: String, reason: String },
    #[error("header buffer too small: expected at least {expected} bytes, found {actual} bytes")]
    HeaderBufferTooSmall { expected: usize, actual: usize },

    // Variable header errors
    #[error("var header not found")]
    VarHeaderNotFound,
    #[error("invalid var header: {0}")]
    InvalidVarHeader(String),
    #[error("invalid var type: {0}")]
    InvalidVarKind(i32),

    // Lookup errors
    #[error("item not found")]
    ItemNotFound,
}
