pub struct TelemtryVars;

impl TelemtryVars {
    /* Engine */
    pub const SPEED: &'static str = "Speed";
    pub const RPM: &'static str = "RPM";
    pub const GEAR: &'static str = "Gear";
    pub const BRAKE_ABS_ACTIVE: &'static str = "BrakeABSactive";

    /* Pedals */
    pub const THROTTLE: &'static str = "Throttle";
    pub const BRAKE: &'static str = "Brake"; // 0=brake released to 1=max pedal force, %
    pub const CLUTCH: &'static str = "Clutch";
    pub const BRAKE_RAW: &'static str = "BrakeRaw"; // Raw brake input 0=brake released to 1=max pedal force, %

    /* Steering */
    pub const STEERING_WHEEL_ANGLE: &'static str = "SteeringWheelAngle";

    /* Laps */
    pub const LAP: &'static str = "Lap";
    pub const LAP_BEST_LAP: &'static str = "LapBestLap"; // Players best lap number,
    pub const LAP_BEST_LAP_TIME: &'static str = "LapBestLapTime"; // Players best lap number,
    pub const LAP_BEST_N_LAP_LAP: &'static str = "LapBestNLapLap"; // Player last lap in best N average lap time,

    /* Forecast */
    pub const AIR_DENSITY: &'static str = "AirDensity";
    pub const AIR_PRESSURE: &'static str = "AirPressure";
    pub const AIR_TEMP: &'static str = "AirTemp";

    /* Camera */
    pub const CAM_CAR_INDEX: &'static str = "CamCarIdx";
    pub const CAM_CAMERA_NUMBER: &'static str = "CamCameraNumber";
    pub const CAM_CAMERA_STATE: &'static str = "CamCameraState"; // State of camera system, irsdk_CameraState
    pub const CAM_GROUP_NUMBER: &'static str = "CamGroupNumber";
}
