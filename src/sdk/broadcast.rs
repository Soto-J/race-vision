#![allow(unused)]

#[cfg(windows)]
use crate::{
    ChatCommandMode, FFBCommandMode, PitCommandMode, ReloadTexturesMode, RpyPosMode, RpySrchMode,
    RpyStateMode, TelemCommandMode, VideoCaptureMode, utils::enums::BroadcastMsg,
};
#[cfg(windows)]
use std::{
    error,
    ffi::{OsStr, c_void},
    io,
    os::windows::ffi::OsStrExt,
    sync::OnceLock,
};
#[cfg(windows)]
use windows::{
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{HWND_BROADCAST, RegisterWindowMessageW, SendNotifyMessageW},
    },
    core::PCWSTR,
};

#[cfg(windows)]
#[derive(Debug)]
pub struct Broadcast {
    msg_id: u32,
}

#[cfg(windows)]
impl Broadcast {
    pub fn new() -> Result<Self, io::Error> {
        let wide: Vec<u16> = OsStr::new("IRSDK_BROADCASTMSG")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let msg_id = unsafe { RegisterWindowMessageW(PCWSTR(wide.as_ptr())) };

        if msg_id == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { msg_id })
    }

    pub fn cam_switch_pos(
        &self,
        position: u32,
        group: Option<u32>,
        camera: Option<u16>,
    ) -> Result<(), windows::core::Error> {
        let group = group.unwrap_or(1);
        let camera = camera.unwrap_or(0);

        self.send(
            BroadcastMsg::CamSwitchPos,
            position,
            Some(group),
            Some(camera),
        )
    }

    pub fn cam_switch_num(
        &self,
        car_number: u32,
        group: Option<u32>,
        camera: Option<u16>,
    ) -> Result<(), windows::core::Error> {
        let group = group.unwrap_or(1);
        let camera = camera.unwrap_or(0);

        self.send(
            BroadcastMsg::CamSwitchNum,
            car_number,
            Some(group),
            Some(camera),
        )
    }

    pub fn cam_set_state(&self, camera_state: u32) -> Result<(), windows::core::Error> {
        self.send(BroadcastMsg::CamSetState, camera_state, None, None)
    }

    pub fn replay_set_play_speed(
        &self,
        speed: u32,
        slow_motion: bool,
    ) -> Result<(), windows::core::Error> {
        let var2 = if slow_motion { 1 } else { 0 };

        self.send(BroadcastMsg::ReplaySetPlaySpeed, speed, Some(var2), None)
    }

    pub fn replay_set_play_position(
        &self,
        pos_mode: Option<RpyPosMode>,
        frame_num: Option<u32>,
    ) -> Result<(), windows::core::Error> {
        let var2 = pos_mode.unwrap_or(RpyPosMode::Begin) as u32;
        let var3 = frame_num.unwrap_or(0);

        self.send(BroadcastMsg::ReplaySetPlayPosition, var2, Some(var3), None)
    }

    pub fn replay_search(
        &self,
        search_mode: Option<RpySrchMode>,
    ) -> Result<(), windows::core::Error> {
        let var2 = search_mode.unwrap_or(RpySrchMode::ToStart) as u32;

        self.send(BroadcastMsg::ReplaySearch, var2, None, None)
    }

    pub fn replay_set_state(
        &self,
        search_mode: Option<RpyStateMode>,
    ) -> Result<(), windows::core::Error> {
        let var2 = search_mode.unwrap_or(RpyStateMode::EraseTape) as u32;

        self.send(BroadcastMsg::ReplaySetState, var2, None, None)
    }

    pub fn reload_all_textures(&self) -> Result<(), windows::core::Error> {
        self.send(
            BroadcastMsg::ReloadTextures,
            ReloadTexturesMode::All as u32,
            None,
            None,
        )
    }

    pub fn reload_texture(&self, car_idx: Option<u32>) -> Result<(), windows::core::Error> {
        use crate::ReloadTexturesMode;

        let var3 = car_idx.unwrap_or(0);

        self.send(
            BroadcastMsg::ReloadTextures,
            ReloadTexturesMode::CarIdx as u32,
            Some(var3),
            None,
        )
    }

    pub fn chat_command(
        &self,
        chat_command_mode: Option<ChatCommandMode>,
    ) -> Result<(), windows::core::Error> {
        let var1 = chat_command_mode.unwrap_or(ChatCommandMode::BeginChat) as u32;

        self.send(BroadcastMsg::ReloadTextures, var1, None, None)
    }

    pub fn chat_command_macro(&self, macro_num: Option<u32>) -> Result<(), windows::core::Error> {
        let var1 = ChatCommandMode::Macro as u32;
        let var2 = macro_num.unwrap_or(0);

        self.send(BroadcastMsg::ChatCommand, var1, Some(var2), None)
    }

    pub fn pit_command(
        &self,
        pit_command_mode: Option<PitCommandMode>,
        var: Option<u32>,
    ) -> Result<(), windows::core::Error> {
        let var1 = pit_command_mode.unwrap_or(PitCommandMode::Clear) as u32;
        let var2 = var.unwrap_or(0);

        self.send(BroadcastMsg::PitCommand, var1, Some(var2), None)
    }

    pub fn telem_command(
        &self,
        telem_command_mode: Option<TelemCommandMode>,
    ) -> Result<(), windows::core::Error> {
        let var1 = telem_command_mode.unwrap_or(TelemCommandMode::Stop) as u32;

        self.send(BroadcastMsg::PitCommand, var1, None, None)
    }

    pub fn ffb_command(
        &self,
        ffb_command_mode: Option<FFBCommandMode>,
        value: Option<u32>,
    ) -> Result<(), windows::core::Error> {
        let var1 = ffb_command_mode.unwrap_or(FFBCommandMode::FfbCommandMaxForce) as u32;
        let var2 = (value.unwrap_or(0) * 65536);

        self.send(BroadcastMsg::FfbCommand, var1, Some(var2), None)
    }

    pub fn replay_search_session_time(
        &self,
        session_num: Option<u16>,
        session_time_ms: Option<u32>,
    ) -> Result<(), windows::core::Error> {
        let var1 = session_num.unwrap_or(0) as u32;
        let var2 = session_time_ms.unwrap_or(0);

        self.send(
            BroadcastMsg::ReplaySearchSessionTime,
            var1,
            Some(var2),
            None,
        )
    }

    pub fn video_capture(
        &self,
        video_capture_mode: Option<VideoCaptureMode>,
    ) -> Result<(), windows::core::Error> {
        let var2 = video_capture_mode.unwrap_or(VideoCaptureMode::TriggerScreenShot) as u32;

        self.send(BroadcastMsg::VideoCapture, var2, None, None)
    }

    fn send(
        &self,
        broadcast_msg_type: BroadcastMsg,
        var1: u32,
        var2: Option<u32>,
        var3: Option<u16>,
    ) -> Result<(), windows::core::Error> {
        let var2 = var2.unwrap_or(0);
        let var3 = var3.unwrap_or(0) as u32;

        unsafe {
            SendNotifyMessageW(
                HWND(0xFFFF as *mut c_void),
                self.msg_id,
                WPARAM((broadcast_msg_type as u32 | (var1 << 16)) as usize),
                LPARAM((var2 | (var3 << 16)) as isize),
            )
        }
    }
}

mod test {
    use tokio::sync::broadcast;

    #[test]
    fn test_cam_switch_pos() {
        // let brodcast = broadcast::new();
    }
    fn test_cam_switch_num() {}
    fn test_replay_set_play_speed() {}
    fn test_replay_set_play_position() {}
    fn test_replay_search() {}
    fn test_replay_set_state() {}
    fn test_reload_all_textures() {}
    fn test_reload_texture() {}
    fn test_chat_command() {}
    fn test_chat_command_macro() {}
    fn test_telem_command() {}
    fn test_ffb_command() {}
    fn test_replay_search_session_time() {}
    fn test_video_capture() {}
}
