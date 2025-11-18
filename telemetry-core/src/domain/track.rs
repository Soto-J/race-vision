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
