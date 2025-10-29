#[derive(Debug, Default, PartialEq)]
pub struct AppState {
    pub iracing_connected: bool,
    pub last_car_setup_tick: i8,
}
