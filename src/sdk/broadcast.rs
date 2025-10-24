#![allow(unused)]

use crate::{BroadcastMsg, utils::BROADCAST_MSG_NAME};

#[cfg(windows)]
use std::error;
use std::{
    ffi::{OsStr, c_void},
    os::windows::ffi::OsStrExt,
    sync::OnceLock,
};
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
    pub fn new() -> Result<Self, std::io::Error> {
        let wide: Vec<u16> = OsStr::new("IRSDK_BROADCASTMSG")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let msg_id = unsafe { RegisterWindowMessageW(PCWSTR(wide.as_ptr())) };

        if msg_id == 0 {
            use std::io;

            return Err(io::Error::last_os_error());
        }

        Ok(Self { msg_id: msg_id })
    }

    pub fn cam_switch_pos(
        &self,
        position: u32,
        group: u32,
        camera: u32,
    ) -> Result<(), windows::core::Error> {
        self.send(BroadcastMsg::CamSwitchPos, position, group, camera)
    }

    pub fn cam_switch_num(
        &self,
        car_number: u32,
        group: u32,
        camera: u32,
    ) -> Result<(), windows::core::Error> {
        self.send(BroadcastMsg::CamSwitchPos, car_number, group, camera)
    }

    fn send(
        &self,
        broadcast_msg_type: BroadcastMsg,
        var1: u32,
        var2: u32,
        var3: u32,
    ) -> Result<(), windows::core::Error> {
        unsafe {
            SendNotifyMessageW(
                HWND(0xFFFF as *mut c_void),
                self.msg_id,
                WPARAM((broadcast_msg_type as u32 | (var1 << 16)) as usize),
                LPARAM((var2 | (var3 << 16)) as isize),
            )
        }
    }

    // fn cam_switch_num(self, position: u32, group: u32, camera: u32) {
    //     self.broadcast_msg(broadcast_type, var1, var2, var3);
    // }
}
