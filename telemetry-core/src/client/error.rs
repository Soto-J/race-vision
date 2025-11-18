use color_eyre::eyre;

#[derive(Debug, thiserror::Error)]
pub enum IRSDKError {
    #[error("Invalid Shared Memory: {0}")]
    InvalidSharedMemory(String),
    #[error("Failed to map memory to view: {0}")]
    FailedToMapView(String),
    #[error("Invalid Handle: {0}")]
    InvalidHandle(String),

    // VarCache errors
    #[error("Header not found")]
    HeaderNotFound,

    // MMap errors
    #[error("View PTR not found")]
    ViewPTRNotFound,
    #[error("View Address not found")]
    ViewAddressNotFound,
    #[error("Memory Snapshot not found")]
    SnapshotNotFound,
    #[error("Data Valid Event not found")]
    DataValidEventNotFound,
    #[error("Var Header not found")]
    VarHeaderNotFound,

    #[error("Invalid Var Header: {0}")]
    InvalidVarHeader(String),
    #[error("Invalid Var Kind: {0} (unknown or unsupported IRSDK var kind)")]
    InvalidVarKind(i32),

    #[error("Item not found")]
    ItemNotFound,
    #[error("Not connected")]
    NotConnected,
    #[error("Time out")]
    Timeout,
    #[error("Unexpected error")]
    UnexpectedError(#[source] eyre::Report),
}

fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
    // tracing::error!("{:?}", eyre::Report::new(e));

    tracing::error!(
        "-----------------------------------------------------------------------------------\n\
         Error: {}\n\
         {:#?}\n\
         -----------------------------------------------------------------------------------",
        e,
        e
    );
}
