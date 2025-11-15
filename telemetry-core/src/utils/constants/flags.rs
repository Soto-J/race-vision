use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EngineWarnings: u8 {
        const WATER_TEMP_WARNING = 0x01;
        const FUEL_PRESSURE_WARNING = 0x02;
        const OIL_PRESSURE_WARNING = 0x04;
        const ENGINE_STALLED = 0x08;
        const PIT_SPEED_LIMITER = 0x10;
        const REV_LIMITER_ACTIVE = 0x20;
        const OIL_TEMP_WARNING = 0x40;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Flags: u32 {
        // global flags
        const CHECKERED        = 0x0001;
        const WHITE            = 0x0002;
        const GREEN            = 0x0004;
        const YELLOW           = 0x0008;
        const RED              = 0x0010;
        const BLUE             = 0x0020;
        const DEBRIS           = 0x0040;
        const CROSSED          = 0x0080;
        const YELLOW_WAVING    = 0x0100;
        const ONE_LAP_TO_GREEN = 0x0200;
        const GREEN_HELD       = 0x0400;
        const TEN_TO_GO        = 0x0800;
        const FIVE_TO_GO       = 0x1000;
        const RANDOM_WAVING    = 0x2000;
        const CAUTION          = 0x4000;
        const CAUTION_WAVING   = 0x8000;

        // drivers black flags
        const BLACK       = 0x010000;
        const DISQUALIFY  = 0x020000;
        const SERVICEABLE = 0x040000; // car is allowed service (not a flag)
        const FURLED      = 0x080000;
        const REPAIR      = 0x100000;

        // start lights
        const START_HIDDEN = 0x10000000;
        const START_READY  = 0x20000000;
        const START_SET    = 0x40000000;
        const START_GO     = 0x80000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct  CameraState: u16 {
        const is_session_screen       = 0x0001; // the camera tool can only be activated if viewing the session screen (out of car)
        const is_scenic_active        = 0x0002; // the scenic camera is active (no focus car)

        // these can be changed with a broadcast message
        const cam_tool_active         = 0x0004;
        const ui_hidden               = 0x0008;
        const use_auto_shot_selection = 0x0010;
        const use_temporary_edits     = 0x0020;
        const use_key_acceleration    = 0x0040;
        const use_key10x_acceleration = 0x0080;
        const use_mouse_aim_mode      = 0x0100;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PitSvFlags: u8 {
        const LF_TIRE_CHANGE     = 0x01;
        const RF_TIRE_CHANGE     = 0x02;
        const LR_TIRE_CHANGE     = 0x04;
        const RR_TIRE_CHANGE     = 0x08;
        const FUEL_FILL          = 0x10;
        const WINDSHIELD_TEAROFF = 0x20;
        const FAST_REPAIR        = 0x40;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct PaceFlags: u8 {
        const end_of_line  = 0x0001;
        const free_pass    = 0x0002;
        const waved_around = 0x0004;
    }
}
