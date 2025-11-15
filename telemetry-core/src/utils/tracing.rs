pub fn init_tracing() {
    tracing_subscriber::fmt()
        .pretty()
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
