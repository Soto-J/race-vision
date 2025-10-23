use windows::{
    Win32::{
        Foundation::{BOOL, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{HWND_BROADCAST, RegisterWindowMessageW, SendNotifyMessageW},
    },
    core::PCWSTR,
};

pub struct Broadcaster {
    msg_id: u32,
}

impl Broadcaster {
    fn new() -> Self {
        let wide: Vec<u16> = "IRSDK_BROADCASTMSG"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let msg_id = unsafe { RegisterWindowMessageW(PCWSTR(wide.as_ptr())) };
        Self { msg_id }
    }

    fn send(&self, broadcast_type: u32, var1: u32, var2: u32, var3: u32) {
        let wparam = WPARAM((broadcast_type | (var1 << 16)) as usize);
        let lparam = LPARAM((var2 | (var3 << 16)) as isize);

        // unsafe { SendNotifyMessageW(HWND_BROADCAST, self.msg_id, wparam, lparam) }

        // unsafe {
        //     let msg_id = broadcast_msg_id();

        //     let wparam = WPARAM((broadcast_type | (var1 << 16)) as usize);
        //     let lparam = LPARAM((var2 | (var3 << 16)) as isize);

        //     SendNotifyMessageW(HWND(0xFFFF), msg_id, wparam, lparam);
        // }
    }
}
