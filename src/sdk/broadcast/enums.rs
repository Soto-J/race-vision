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

pub enum RpyPosMode {
    Begin = 0,
    Current = 1,
    End = 2,
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

pub enum RpyStateMode {
    EraseTape = 0, // clear any data in the replay tape
}

pub enum FFBCommandMode {
    FfbCommandMaxForce = 0, // Set the maximum force when mapping steering torque force to direct input units (float in Nm)
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

pub enum ReloadTexturesMode {
    All = 0,    // reload all textuers
    CarIdx = 1, // reload only textures for the specific carIdx
}

pub enum ChatCommandMode {
    Macro = 0,     // pass in a number from 1-15 representing the chat macro to launch
    BeginChat = 1, // Open up a new chat window
    Reply = 2,     // Reply to last private chat
    Cancel = 3,    // Close chat window
}

// You can call this any time, but telemtry only records when driver is in there car
pub enum TelemCommandMode {
    Stop = 0,    // Turn telemetry recording off
    Start = 1,   // Turn telemetry recording on
    Restart = 2, // Write current file to disk and start a new one
}

pub enum VideoCaptureMode {
    TriggerScreenShot = 0,  // save a screenshot to disk
    StartVideoCapture = 1,  // start capturing video
    EndVideoCapture = 2,    // stop capturing video
    ToggleVideoCapture = 3, // toggle video capture on/off
    ShowVideoTimer = 4,     // show video timer in upper left corner of display
    HideVideoTimer = 5,     // hide video timer
}
