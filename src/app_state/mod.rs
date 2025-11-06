// Will need when implementing tauri
#[derive(Debug, Default, PartialEq)]
pub struct AppState {
    pub irsdk: Arc<Mutex<IRSDK>>,
    pub connection_status: Arc<Mutex<ConnectionStatus>>,
    pub telemetry_cache: Arc<Mutex<HashMap<String, VarData>>>,
}
