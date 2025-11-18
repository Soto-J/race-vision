use crate::{
    client::telemetry::models::{Header, VarHeader},
    utils::constants::telemetry_vars::TelemetryVars,
};

use std::{collections::HashMap, fs::File};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct SessionState {
    pub last_session_info_update: u64,
    pub session_info_hash: HashMap<String, VarHeader>,
    pub broadcast_msg_id: Option<u32>,
    pub workaround_state: WorkaroundState,
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

        self.workaround_state = match (self.workaround_state, status) {
            (WorkaroundState::Idle, status) if status != connected => {
                WorkaroundState::ObservedDisconnected
            }
            (WorkaroundState::ObservedDisconnected, _)
                if !has_session_num || test_file.is_some() =>
            {
                WorkaroundState::WaitingForSessionNum
            }
            (WorkaroundState::WaitingForSessionNum, _)
                if var_headers_hash.contains_key(TelemetryVars::SESSION_NUM) =>
            {
                WorkaroundState::WorkaroundConfirmed
            }
            (_, status) if status == connected => WorkaroundState::Idle,
            (state, _) => state,
        };

        let status_is_connected = status == connected;
        let is_workaround_connected = self.workaround_state == WorkaroundState::WorkaroundConfirmed;

        let has_data_source = test_file.is_some() || has_data_valid_event;

        has_data_source && (status_is_connected || is_workaround_connected)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum WorkaroundState {
    #[default]
    Idle,
    ObservedDisconnected, // We (observed) a status that was NOT connected.
    WaitingForSessionNum,
    WorkaroundConfirmed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusField {
    StatusConnected = 1,
}
