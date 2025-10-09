use bitflags::bitflags;

mod sdk;
mod ibt;

pub enum StatusField {
    StatusConnected = 1,
}

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
        const BLACK      = 0x010000;
        const DISQUALIFY = 0x020000;
        const SERVICIBLE = 0x040000; // car is allowed service (not a flag)
        const FURLED     = 0x080000;
        const REPAIR     = 0x100000;

        // start lights
        const START_HIDDEN = 0x10000000;
        const START_READY  = 0x20000000;
        const START_SET    = 0x40000000;
        const START_GO     = 0x80000000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrkLoc {
    NotInWorld = -1,
    OffTrack = 0,
    InPitStall = 1,
    ApproachingPits = 2,
    OnTrack = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrkSurf {
    NotInWorld = -1,
    Undefined = 0,
    Asphalt1 = 1,
    Asphalt2 = 2,
    Asphalt3 = 3,
    Asphalt4 = 4,
    Concrete1 = 5,
    Concrete2 = 6,
    RacingDirt1 = 7,
    RacingDirt2 = 8,
    Paint1 = 9,
    Paint2 = 10,
    Rumble1 = 11,
    Rumble2 = 12,
    Rumble3 = 13,
    Rumble4 = 14,
    Grass1 = 15,
    Grass2 = 16,
    Grass3 = 17,
    Grass4 = 18,
    Dirt1 = 19,
    Dirt2 = 20,
    Dirt3 = 21,
    Dirt4 = 22,
    Sand = 23,
    Gravel1 = 24,
    Gravel2 = 25,
    GRASSCRETE = 26,
    Astroturf = 27,
}

pub enum SessionState {
    Invalid = 0,
    GetInCar = 1,
    Warmup = 2,
    ParadeLaps = 3,
    Racing = 4,
    Checkered = 5,
    CoolDown = 6,
}

bitflags! {
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

pub enum BroadcastMsg {
    CamSwitchPos = 0,             // car position, group, camera
    CamSwitchNum = 1,             // driver #, group, camera
    CamSetState = 2,              // CameraState, unused, unused
    ReplaySetPlaySpeed = 3,       // speed, slowMotion, unused
    ReplaySetPlayPosition = 4,    // RpyPosMode, Frame Number (high, low)
    ReplaySearch = 5,             // RpySrchMode, unused, unused
    ReplaySetState = 6,           // RpyStateMode, unused, unused
    ReloadTextures = 7,           // ReloadTexturesMode, carIdx, unused
    ChatCommand = 8,              // ChatCommandMode, subCommand, unused
    PitCommand = 9,               // PitCommandMode, parameter
    TelemCommand = 10,            // irsdk_TelemCommandMode, unused, unused
    FfbCommand = 11,              // irsdk_FFBCommandMode, value (float, high, low)
    ReplaySearchSessionTime = 12, // sessionNum, sessionTimeMS (high, low)
    VideoCapture = 13,            // irsdk_VideoCaptureMode, unused, unused
}

pub enum ChatCommandMode {
    Macro = 0,     // pass in a number from 1-15 representing the chat macro to launch
    BeginChat = 1, // Open up a new chat window
    Reply = 2,     // Reply to last private chat
    Cancel = 3,    // Close chat window
}

// this only works when the driver is in the car
pub enum PitCommandMode {
    Clear = 0,      // Clear all pit checkboxes
    Ws = 1,         // Clean the winshield, using one tear off
    Fuel = 2, // Add fuel, optionally specify the amount to add in liters or pass '0' to use existing amount
    Lf = 3, // Change the left front tire, optionally specifying the pressure in KPa or pass '0' to use existing pressure
    Rf = 4, // right front
    Lr = 5, // left rear
    Rr = 6, // right rear
    ClearTires = 7, // Clear tire pit checkboxes
    Fr = 8, // Request a fast repair
    ClearWs = 9, // Uncheck Clean the winshield checkbox
    ClearFr = 10, // Uncheck request a fast repair
    ClearFuel = 11, // Uncheck add fuel
}

// You can call this any time, but telemtry only records when driver is in there car
pub enum TelemCommandMode {
    Stop = 0,    // Turn telemetry recording off
    Start = 1,   // Turn telemetry recording on
    Restart = 2, // Write current file to disk and start a new one
}

pub enum RpyStateMode {
    EraseTape = 0, // clear any data in the replay tape
}

pub enum ReloadTexturesMode {
    All = 0,    // reload all textuers
    CarIdx = 1, // reload only textures for the specific carIdx
}

pub enum RpySrchMode {
    ToStart = 0,
    ToEnd = 1,
    PrevSession = 2,
    NextSession = 3,
    PrevLap = 4,
    NextLap = 5,
    PrevFrame = 6,
    NextFrame = 7,
    PrevIncident = 8,
    NextIncident = 9,
}

pub enum RpyPosMode {
    Begin = 0,
    Current = 1,
    End = 2,
}

pub enum CsMode {
    AtIncident = -3,
    AtLeader = -2,
    AtExciting = -1,
}

bitflags! {
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

pub enum PitSvStatus {
    // status
    None = 0,
    InProgress = 1,
    Complete = 2,

    // errors
    TooFarLeft = 100,
    TooFarRight = 101,
    TooFarForward = 102,
    TooFarBack = 103,
    BadAngle = 104,
    CantFixThat = 105,
}

pub enum PaceMode {
    SingleFileStart = 0,
    DoubleFileStart = 1,
    SingleFileRestart = 2,
    DoubleFileRestart = 3,
    NotPacing = 4,
}

bitflags! {
    struct PaceFlags: u8 {
        const end_of_line  = 0x0001;
        const free_pass    = 0x0002;
        const waved_around = 0x0004;
    }
}

pub enum CarLeftRight {
    Off = 0,
    Clear = 1,        // no cars around us.
    CarLeft = 2,      // there is a car to our left.
    CarRight = 3,     // there is a car to our right.
    CarLeftRight = 4, // there are cars on each side.
    TwoCarsLeft = 5,  // there are two cars to our left.
    TwoCarsRight = 6, // there are two cars to our right.
}

// You can call this any time
pub enum FFBCommandMode {
    FfbCommandMaxForce = 0, // Set the maximum force when mapping steering torque force to direct input units (float in Nm)
}

pub enum VideoCaptureMode {
    TriggerScreenShot = 0,  // save a screenshot to disk
    StartVideoCapture = 1,  // start capturing video
    EndVideoCapture = 2,    // stop capturing video
    ToggleVideoCapture = 3, // toggle video capture on/off
    ShowVideoTimer = 4,     // show video timer in upper left corner of display
    HideVideoTimer = 5,     // hide video timer
}

pub enum TrackWetness {
    Unknown = 0,
    Dry = 1,
    MostlyDry = 2,
    VeryLightlyWet = 3,
    LightlyWet = 4,
    ModeratelyWet = 5,
    VeryWet = 6,
    ExtremelyWet = 7,
}
