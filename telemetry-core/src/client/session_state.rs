use crate::{
    client::telemetry::models::{Header, VarHeader},
    utils::{constants::telemetry_vars::TelemetryVars, enums::StatusField},
};

use std::{collections::HashMap, fs::File};

#[repr(C)]
#[derive(Debug, Default)]
pub struct SessionState {
    pub last_session_info_update: u64,
    pub session_info_hash: HashMap<String, VarHeader>,
    pub broadcast_msg_id: Option<u32>,
    pub workaround_connected_state: u16,
}

impl SessionState {
    fn is_connected(
        &mut self,
        header: Option<&Header>,
        var_headers_hash: &HashMap<String, VarHeader>,
        test_file: Option<&File>,
        has_data_valid_event: bool,
    ) -> bool {
        let Some(header) = &header else {
            return false;
        };

        let status = header.status();
        let connected = StatusField::StatusConnected as i32;
        let has_session_num = var_headers_hash.contains_key(TelemetryVars::SESSION_NUM);

        self.workaround_connected_state = match (self.workaround_connected_state, status) {
            (0, status) if status != connected => 1,
            (1, _) if !has_session_num || test_file.is_some() => 2,
            (2, _) if var_headers_hash.contains_key(TelemetryVars::SESSION_NUM) => 3,
            (_, status) if status == connected => 0,
            (state, _) => state,
        };

        let is_status_connected = status == connected;
        let is_workaround_connected = self.workaround_connected_state == 3;
        let has_data_source = test_file.is_some() || has_data_valid_event;

        has_data_source && (is_status_connected || is_workaround_connected)
    }
}
