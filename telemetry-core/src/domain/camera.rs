pub enum CsMode {
    AtIncident = -3,
    AtLeader = -2,
    AtExciting = -1,
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
