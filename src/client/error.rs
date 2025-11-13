use color_eyre::eyre;

#[derive(Debug, thiserror::Error)]
pub enum IRSDKError {
    #[error("Invalid Shared Memory: {0}")]
    InvalidSharedMemory(String),
    #[error("Failed to map memory to view: {0}")]
    FailedToMapView(String),
    #[error("Invalid Handle")]
    InvalidHandle,

    #[error("Invalid Var Header: {0}")]
    InvalidVarHeader(String),
    #[error("Invalid Var Type: {0} (unknown or unsupported IRSDK var type)")]
    InvalidVarType(i32),

    #[error("Item Not Found")]
    ItemNotFound,
    #[error("Not Connected")]
    NotConnected,
    #[error("Time out")]
    Timeout,
    #[error("Unexpected Error")]
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
