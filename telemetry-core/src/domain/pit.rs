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