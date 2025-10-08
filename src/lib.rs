use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EnginWarnings: u8 {
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
    UNDEFINED = 0,
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


// pub enum Session