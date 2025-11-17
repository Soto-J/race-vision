pub mod var_types;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusField {
    StatusConnected = 1,
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

pub enum CsMode {
    AtIncident = -3,
    AtLeader = -2,
    AtExciting = -1,
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
