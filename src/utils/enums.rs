#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRacingVarType {
    Char = 0,
    Bool = 1,
    Int = 2,
    Bitfield = 3,
    Float = 4,
    Double = 5,
}

impl TryFrom<i32> for IRacingVarType {
    type Error = String;

    fn try_from(var_type: i32) -> Result<Self, Self::Error> {
        match var_type {
            0 => Ok(Self::Char),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Int),
            3 => Ok(Self::Bitfield),
            4 => Ok(Self::Float),
            5 => Ok(Self::Double),
            _ => Err(format!("Unknown variable type: {}", var_type)),
        }
    }
}

// Enum to represent telemetry values
#[derive(Debug, Clone, PartialEq)]
pub enum VarData {
    Chars(Vec<u8>),
    Bools(Vec<bool>),
    Int(Vec<i32>),
    Bitfields(Vec<u32>),
    Floats(Vec<f32>),
    Doubles(Vec<f64>),
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

pub enum IracingVars {}
