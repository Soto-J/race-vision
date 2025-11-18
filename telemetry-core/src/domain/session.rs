pub enum SessionState {
    Invalid = 0,
    GetInCar = 1,
    Warmup = 2,
    ParadeLaps = 3,
    Racing = 4,
    Checkered = 5,
    CoolDown = 6,
}

pub enum PaceMode {
    SingleFileStart = 0,
    DoubleFileStart = 1,
    SingleFileRestart = 2,
    DoubleFileRestart = 3,
    NotPacing = 4,
}
