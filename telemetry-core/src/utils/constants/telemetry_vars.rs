pub struct TelemetryVars;

impl TelemetryVars {
    /* Environmental */
    pub const AIR_DENSITY: &'static str = "AirDensity";
    pub const AIR_PRESSURE: &'static str = "AirPressure";
    pub const AIR_TEMP: &'static str = "AirTemp";
    pub const FOG_LEVEL: &'static str = "FogLevel";
    pub const PRECIPITATION: &'static str = "Precipitation";
    pub const RELATIVE_HUMIDITY: &'static str = "RelativeHumidity";
    pub const SKIES: &'static str = "Skies";
    pub const SOLAR_ALTITUDE: &'static str = "SolarAltitude";
    pub const SOLAR_AZIMUTH: &'static str = "SolarAzimuth";
    pub const TRACK_TEMP: &'static str = "TrackTemp";
    pub const TRACK_TEMP_CREW: &'static str = "TrackTempCrew";
    pub const TRACK_WETNESS: &'static str = "TrackWetness";
    pub const WEATHER_DECLARED_WET: &'static str = "WeatherDeclaredWet";
    pub const WIND_DIR: &'static str = "WindDir";
    pub const WIND_VEL: &'static str = "WindVel";

    /* Pedals */
    pub const BRAKE: &'static str = "Brake";
    pub const BRAKE_ABS_ACTIVE: &'static str = "BrakeABSactive";
    pub const BRAKE_RAW: &'static str = "BrakeRaw";
    pub const CLUTCH: &'static str = "Clutch";
    pub const CLUTCH_RAW: &'static str = "ClutchRaw";
    pub const HANDBRAKE_RAW: &'static str = "HandbrakeRaw";
    pub const THROTTLE: &'static str = "Throttle";
    pub const THROTTLE_RAW: &'static str = "ThrottleRaw";

    /* Camera */
    pub const CAM_CAMERA_NUMBER: &'static str = "CamCameraNumber";
    pub const CAM_CAMERA_STATE: &'static str = "CamCameraState";
    pub const CAM_CAR_IDX: &'static str = "CamCarIdx";
    pub const CAM_GROUP_NUMBER: &'static str = "CamGroupNumber";

    /* Car Position/Distance */
    pub const CAR_DIST_AHEAD: &'static str = "CarDistAhead";
    pub const CAR_DIST_BEHIND: &'static str = "CarDistBehind";
    pub const CAR_LEFT_RIGHT: &'static str = "CarLeftRight";

    /* Car Index Arrays - Best Lap */
    pub const CAR_IDX_BEST_LAP_NUM: &'static str = "CarIdxBestLapNum";
    pub const CAR_IDX_BEST_LAP_TIME: &'static str = "CarIdxBestLapTime";

    /* Car Index Arrays - Class/Position */
    pub const CAR_IDX_CLASS: &'static str = "CarIdxClass";
    pub const CAR_IDX_CLASS_POSITION: &'static str = "CarIdxClassPosition";
    pub const CAR_IDX_POSITION: &'static str = "CarIdxPosition";

    /* Car Index Arrays - Timing */
    pub const CAR_IDX_EST_TIME: &'static str = "CarIdxEstTime";
    pub const CAR_IDX_F2_TIME: &'static str = "CarIdxF2Time";
    pub const CAR_IDX_LAST_LAP_TIME: &'static str = "CarIdxLastLapTime";

    /* Car Index Arrays - Repairs */
    pub const CAR_IDX_FAST_REPAIRS_USED: &'static str = "CarIdxFastRepairsUsed";

    /* Car Index Arrays - Gear/RPM */
    pub const CAR_IDX_GEAR: &'static str = "CarIdxGear";
    pub const CAR_IDX_RPM: &'static str = "CarIdxRPM";

    /* Car Index Arrays - Laps */
    pub const CAR_IDX_LAP: &'static str = "CarIdxLap";
    pub const CAR_IDX_LAP_COMPLETED: &'static str = "CarIdxLapCompleted";
    pub const CAR_IDX_LAP_DIST_PCT: &'static str = "CarIdxLapDistPct";

    /* Car Index Arrays - Pit/Track */
    pub const CAR_IDX_ON_PIT_ROAD: &'static str = "CarIdxOnPitRoad";
    pub const CAR_IDX_TRACK_SURFACE: &'static str = "CarIdxTrackSurface";
    pub const CAR_IDX_TRACK_SURFACE_MATERIAL: &'static str = "CarIdxTrackSurfaceMaterial";

    /* Car Index Arrays - Push to Pass */
    pub const CAR_IDX_P2P_COUNT: &'static str = "CarIdxP2P_Count";
    pub const CAR_IDX_P2P_STATUS: &'static str = "CarIdxP2P_Status";

    /* Car Index Arrays - Pacing */
    pub const CAR_IDX_PACE_FLAGS: &'static str = "CarIdxPaceFlags";
    pub const CAR_IDX_PACE_LINE: &'static str = "CarIdxPaceLine";
    pub const CAR_IDX_PACE_ROW: &'static str = "CarIdxPaceRow";

    /* Car Index Arrays - Tires */
    pub const CAR_IDX_QUAL_TIRE_COMPOUND: &'static str = "CarIdxQualTireCompound";
    pub const CAR_IDX_QUAL_TIRE_COMPOUND_LOCKED: &'static str = "CarIdxQualTireCompoundLocked";
    pub const CAR_IDX_TIRE_COMPOUND: &'static str = "CarIdxTireCompound";

    /* Car Index Arrays - Flags/Steering */
    pub const CAR_IDX_SESSION_FLAGS: &'static str = "CarIdxSessionFlags";
    pub const CAR_IDX_STEER: &'static str = "CarIdxSteer";

    /* Communications */
    pub const CHAN_AVG_LATENCY: &'static str = "ChanAvgLatency";
    pub const CHAN_CLOCK_SKEW: &'static str = "ChanClockSkew";
    pub const CHAN_LATENCY: &'static str = "ChanLatency";
    pub const CHAN_PARTNER_QUALITY: &'static str = "ChanPartnerQuality";
    pub const CHAN_QUALITY: &'static str = "ChanQuality";

    /* Performance Monitoring */
    pub const CPU_USAGE_BG: &'static str = "CpuUsageBG";
    pub const CPU_USAGE_FG: &'static str = "CpuUsageFG";
    pub const FRAME_RATE: &'static str = "FrameRate";
    pub const GPU_USAGE: &'static str = "GpuUsage";
    pub const MEM_PAGE_FAULT_SEC: &'static str = "MemPageFaultSec";
    pub const MEM_SOFT_PAGE_FAULT_SEC: &'static str = "MemSoftPageFaultSec";

    /* Driver Change */
    pub const DC_DRIVERS_SO_FAR: &'static str = "DCDriversSoFar";
    pub const DC_LAP_STATUS: &'static str = "DCLapStatus";

    /* In-Car Controls */
    pub const DC_PIT_SPEED_LIMITER_TOGGLE: &'static str = "dcPitSpeedLimiterToggle";
    pub const DC_STARTER: &'static str = "dcStarter";
    pub const DC_TOGGLE_WINDSHIELD_WIPERS: &'static str = "dcToggleWindshieldWipers";
    pub const DC_TRIGGER_WINDSHIELD_WIPERS: &'static str = "dcTriggerWindshieldWipers";

    /* Settings */
    pub const DISPLAY_UNITS: &'static str = "DisplayUnits";

    /* Pitstop Settings */
    pub const DP_FAST_REPAIR: &'static str = "dpFastRepair";
    pub const DP_FUEL_ADD_KG: &'static str = "dpFuelAddKg";
    pub const DP_FUEL_AUTO_FILL_ACTIVE: &'static str = "dpFuelAutoFillActive";
    pub const DP_FUEL_AUTO_FILL_ENABLED: &'static str = "dpFuelAutoFillEnabled";
    pub const DP_FUEL_FILL: &'static str = "dpFuelFill";
    pub const DP_LF_TIRE_CHANGE: &'static str = "dpLFTireChange";
    pub const DP_LF_TIRE_COLD_PRESS: &'static str = "dpLFTireColdPress";
    pub const DP_LR_TIRE_CHANGE: &'static str = "dpLRTireChange";
    pub const DP_LR_TIRE_COLD_PRESS: &'static str = "dpLRTireColdPress";
    pub const DP_RF_TIRE_CHANGE: &'static str = "dpRFTireChange";
    pub const DP_RF_TIRE_COLD_PRESS: &'static str = "dpRFTireColdPress";
    pub const DP_RR_TIRE_CHANGE: &'static str = "dpRRTireChange";
    pub const DP_RR_TIRE_COLD_PRESS: &'static str = "dpRRTireColdPress";
    pub const DP_WINDSHIELD_TEAROFF: &'static str = "dpWindshieldTearoff";

    /* Driver/Car State */
    pub const DRIVER_MARKER: &'static str = "DriverMarker";

    /* Engine */
    pub const ENGINE0_RPM: &'static str = "Engine0_RPM";
    pub const ENGINE_WARNINGS: &'static str = "EngineWarnings";
    pub const FUEL_LEVEL: &'static str = "FuelLevel";
    pub const FUEL_LEVEL_PCT: &'static str = "FuelLevelPct";
    pub const FUEL_PRESS: &'static str = "FuelPress";
    pub const FUEL_USE_PER_HOUR: &'static str = "FuelUsePerHour";
    pub const GEAR: &'static str = "Gear";
    pub const MANIFOLD_PRESS: &'static str = "ManifoldPress";
    pub const OIL_LEVEL: &'static str = "OilLevel";
    pub const OIL_PRESS: &'static str = "OilPress";
    pub const OIL_TEMP: &'static str = "OilTemp";
    pub const RPM: &'static str = "RPM";
    pub const SHIFT_GRIND_RPM: &'static str = "ShiftGrindRPM";
    pub const SHIFT_INDICATOR_PCT: &'static str = "ShiftIndicatorPct";
    pub const SHIFT_POWER_PCT: &'static str = "ShiftPowerPct";
    pub const VOLTAGE: &'static str = "Voltage";
    pub const WATER_LEVEL: &'static str = "WaterLevel";
    pub const WATER_TEMP: &'static str = "WaterTemp";

    /* Hybrid */
    pub const MANUAL_BOOST: &'static str = "ManualBoost";
    pub const MANUAL_NO_BOOST: &'static str = "ManualNoBoost";

    /* Session Control */
    pub const ENTER_EXIT_RESET: &'static str = "EnterExitReset";

    /* Repairs */
    pub const FAST_REPAIR_AVAILABLE: &'static str = "FastRepairAvailable";
    pub const FAST_REPAIR_USED: &'static str = "FastRepairUsed";

    /* Tire Sets */
    pub const FRONT_TIRE_SETS_AVAILABLE: &'static str = "FrontTireSetsAvailable";
    pub const FRONT_TIRE_SETS_USED: &'static str = "FrontTireSetsUsed";
    pub const LEFT_TIRE_SETS_AVAILABLE: &'static str = "LeftTireSetsAvailable";
    pub const LEFT_TIRE_SETS_USED: &'static str = "LeftTireSetsUsed";
    pub const LF_TIRES_AVAILABLE: &'static str = "LFTiresAvailable";
    pub const LF_TIRES_USED: &'static str = "LFTiresUsed";
    pub const LR_TIRES_AVAILABLE: &'static str = "LRTiresAvailable";
    pub const LR_TIRES_USED: &'static str = "LRTiresUsed";
    pub const REAR_TIRE_SETS_AVAILABLE: &'static str = "RearTireSetsAvailable";
    pub const REAR_TIRE_SETS_USED: &'static str = "RearTireSetsUsed";
    pub const RF_TIRES_AVAILABLE: &'static str = "RFTiresAvailable";
    pub const RF_TIRES_USED: &'static str = "RFTiresUsed";
    pub const RIGHT_TIRE_SETS_AVAILABLE: &'static str = "RightTireSetsAvailable";
    pub const RIGHT_TIRE_SETS_USED: &'static str = "RightTireSetsUsed";
    pub const RR_TIRES_AVAILABLE: &'static str = "RRTiresAvailable";
    pub const RR_TIRES_USED: &'static str = "RRTiresUsed";
    pub const TIRE_SETS_AVAILABLE: &'static str = "TireSetsAvailable";
    pub const TIRE_SETS_USED: &'static str = "TireSetsUsed";

    /* Telemetry System */
    pub const IS_DISK_LOGGING_ACTIVE: &'static str = "IsDiskLoggingActive";
    pub const IS_DISK_LOGGING_ENABLED: &'static str = "IsDiskLoggingEnabled";

    /* Garage/Track State */
    pub const IS_GARAGE_VISIBLE: &'static str = "IsGarageVisible";
    pub const IS_IN_GARAGE: &'static str = "IsInGarage";
    pub const IS_ON_TRACK: &'static str = "IsOnTrack";
    pub const IS_ON_TRACK_CAR: &'static str = "IsOnTrackCar";
    pub const IS_REPLAY_PLAYING: &'static str = "IsReplayPlaying";

    /* Lap Timing */
    pub const LAP: &'static str = "Lap";
    pub const LAP_BEST_LAP: &'static str = "LapBestLap";
    pub const LAP_BEST_LAP_TIME: &'static str = "LapBestLapTime";
    pub const LAP_BEST_N_LAP_LAP: &'static str = "LapBestNLapLap";
    pub const LAP_BEST_N_LAP_TIME: &'static str = "LapBestNLapTime";
    pub const LAP_COMPLETED: &'static str = "LapCompleted";
    pub const LAP_CURRENT_LAP_TIME: &'static str = "LapCurrentLapTime";
    pub const LAP_DELTA_TO_BEST_LAP: &'static str = "LapDeltaToBestLap";
    pub const LAP_DELTA_TO_BEST_LAP_DD: &'static str = "LapDeltaToBestLap_DD";
    pub const LAP_DELTA_TO_BEST_LAP_OK: &'static str = "LapDeltaToBestLap_OK";
    pub const LAP_DELTA_TO_OPTIMAL_LAP: &'static str = "LapDeltaToOptimalLap";
    pub const LAP_DELTA_TO_OPTIMAL_LAP_DD: &'static str = "LapDeltaToOptimalLap_DD";
    pub const LAP_DELTA_TO_OPTIMAL_LAP_OK: &'static str = "LapDeltaToOptimalLap_OK";
    pub const LAP_DELTA_TO_SESSION_BEST_LAP: &'static str = "LapDeltaToSessionBestLap";
    pub const LAP_DELTA_TO_SESSION_BEST_LAP_DD: &'static str = "LapDeltaToSessionBestLap_DD";
    pub const LAP_DELTA_TO_SESSION_BEST_LAP_OK: &'static str = "LapDeltaToSessionBestLap_OK";
    pub const LAP_DELTA_TO_SESSION_LASTL_LAP: &'static str = "LapDeltaToSessionLastlLap";
    pub const LAP_DELTA_TO_SESSION_LASTL_LAP_DD: &'static str = "LapDeltaToSessionLastlLap_DD";
    pub const LAP_DELTA_TO_SESSION_LASTL_LAP_OK: &'static str = "LapDeltaToSessionLastlLap_OK";
    pub const LAP_DELTA_TO_SESSION_OPTIMAL_LAP: &'static str = "LapDeltaToSessionOptimalLap";
    pub const LAP_DELTA_TO_SESSION_OPTIMAL_LAP_DD: &'static str = "LapDeltaToSessionOptimalLap_DD";
    pub const LAP_DELTA_TO_SESSION_OPTIMAL_LAP_OK: &'static str = "LapDeltaToSessionOptimalLap_OK";
    pub const LAP_DIST: &'static str = "LapDist";
    pub const LAP_DIST_PCT: &'static str = "LapDistPct";
    pub const LAP_LAS_N_LAP_SEQ: &'static str = "LapLasNLapSeq";
    pub const LAP_LAST_LAP_TIME: &'static str = "LapLastLapTime";
    pub const LAP_LAST_N_LAP_TIME: &'static str = "LapLastNLapTime";

    /* Acceleration */
    pub const LAT_ACCEL: &'static str = "LatAccel";
    pub const LAT_ACCEL_ST: &'static str = "LatAccel_ST";
    pub const LONG_ACCEL: &'static str = "LongAccel";
    pub const LONG_ACCEL_ST: &'static str = "LongAccel_ST";
    pub const VERT_ACCEL: &'static str = "VertAccel";
    pub const VERT_ACCEL_ST: &'static str = "VertAccel_ST";

    /* Left Front Tire */
    pub const LF_BRAKE_LINE_PRESS: &'static str = "LFbrakeLinePress";
    pub const LF_COLD_PRESSURE: &'static str = "LFcoldPressure";
    pub const LF_ODOMETER: &'static str = "LFodometer";
    pub const LF_SHOCK_DEFL: &'static str = "LFshockDefl";
    pub const LF_SHOCK_DEFL_ST: &'static str = "LFshockDefl_ST";
    pub const LF_SHOCK_VEL: &'static str = "LFshockVel";
    pub const LF_SHOCK_VEL_ST: &'static str = "LFshockVel_ST";
    pub const LF_TEMP_CL: &'static str = "LFtempCL";
    pub const LF_TEMP_CM: &'static str = "LFtempCM";
    pub const LF_TEMP_CR: &'static str = "LFtempCR";
    pub const LF_WEAR_L: &'static str = "LFwearL";
    pub const LF_WEAR_M: &'static str = "LFwearM";
    pub const LF_WEAR_R: &'static str = "LFwearR";

    /* Left Rear Tire */
    pub const LR_BRAKE_LINE_PRESS: &'static str = "LRbrakeLinePress";
    pub const LR_COLD_PRESSURE: &'static str = "LRcoldPressure";
    pub const LR_ODOMETER: &'static str = "LRodometer";
    pub const LR_SHOCK_DEFL: &'static str = "LRshockDefl";
    pub const LR_SHOCK_DEFL_ST: &'static str = "LRshockDefl_ST";
    pub const LR_SHOCK_VEL: &'static str = "LRshockVel";
    pub const LR_SHOCK_VEL_ST: &'static str = "LRshockVel_ST";
    pub const LR_TEMP_CL: &'static str = "LRtempCL";
    pub const LR_TEMP_CM: &'static str = "LRtempCM";
    pub const LR_TEMP_CR: &'static str = "LRtempCR";
    pub const LR_WEAR_L: &'static str = "LRwearL";
    pub const LR_WEAR_M: &'static str = "LRwearM";
    pub const LR_WEAR_R: &'static str = "LRwearR";

    /* Right Front Tire */
    pub const RF_BRAKE_LINE_PRESS: &'static str = "RFbrakeLinePress";
    pub const RF_COLD_PRESSURE: &'static str = "RFcoldPressure";
    pub const RF_ODOMETER: &'static str = "RFodometer";
    pub const RF_SHOCK_DEFL: &'static str = "RFshockDefl";
    pub const RF_SHOCK_DEFL_ST: &'static str = "RFshockDefl_ST";
    pub const RF_SHOCK_VEL: &'static str = "RFshockVel";
    pub const RF_SHOCK_VEL_ST: &'static str = "RFshockVel_ST";
    pub const RF_TEMP_CL: &'static str = "RFtempCL";
    pub const RF_TEMP_CM: &'static str = "RFtempCM";
    pub const RF_TEMP_CR: &'static str = "RFtempCR";
    pub const RF_WEAR_L: &'static str = "RFwearL";
    pub const RF_WEAR_M: &'static str = "RFwearM";
    pub const RF_WEAR_R: &'static str = "RFwearR";

    /* Right Rear Tire */
    pub const RR_BRAKE_LINE_PRESS: &'static str = "RRbrakeLinePress";
    pub const RR_COLD_PRESSURE: &'static str = "RRcoldPressure";
    pub const RR_ODOMETER: &'static str = "RRodometer";
    pub const RR_SHOCK_DEFL: &'static str = "RRshockDefl";
    pub const RR_SHOCK_DEFL_ST: &'static str = "RRshockDefl_ST";
    pub const RR_SHOCK_VEL: &'static str = "RRshockVel";
    pub const RR_SHOCK_VEL_ST: &'static str = "RRshockVel_ST";
    pub const RR_TEMP_CL: &'static str = "RRtempCL";
    pub const RR_TEMP_CM: &'static str = "RRtempCM";
    pub const RR_TEMP_CR: &'static str = "RRtempCR";
    pub const RR_WEAR_L: &'static str = "RRwearL";
    pub const RR_WEAR_M: &'static str = "RRwearM";
    pub const RR_WEAR_R: &'static str = "RRwearR";

    /* Tire Rumble */
    pub const TIRE_LF_RUMBLE_PITCH: &'static str = "TireLF_RumblePitch";
    pub const TIRE_LR_RUMBLE_PITCH: &'static str = "TireLR_RumblePitch";
    pub const TIRE_RF_RUMBLE_PITCH: &'static str = "TireRF_RumblePitch";
    pub const TIRE_RR_RUMBLE_PITCH: &'static str = "TireRR_RumblePitch";

    /* Textures */
    pub const LOAD_NUM_TEXTURES: &'static str = "LoadNumTextures";
    pub const OK_TO_RELOAD_TEXTURES: &'static str = "OkToReloadTextures";

    /* Pit Road */
    pub const ON_PIT_ROAD: &'static str = "OnPitRoad";

    /* Push to Pass */
    pub const P2P_COUNT: &'static str = "P2P_Count";
    pub const P2P_STATUS: &'static str = "P2P_Status";
    pub const PUSH_TO_PASS: &'static str = "PushToPass";
    pub const PUSH_TO_TALK: &'static str = "PushToTalk";

    /* Pace Mode */
    pub const PACE_MODE: &'static str = "PaceMode";

    /* Orientation */
    pub const PITCH: &'static str = "Pitch";
    pub const PITCH_RATE: &'static str = "PitchRate";
    pub const PITCH_RATE_ST: &'static str = "PitchRate_ST";
    pub const ROLL: &'static str = "Roll";
    pub const ROLL_RATE: &'static str = "RollRate";
    pub const ROLL_RATE_ST: &'static str = "RollRate_ST";
    pub const YAW: &'static str = "Yaw";
    pub const YAW_NORTH: &'static str = "YawNorth";
    pub const YAW_RATE: &'static str = "YawRate";
    pub const YAW_RATE_ST: &'static str = "YawRate_ST";

    /* Pit Service */
    pub const PIT_OPT_REPAIR_LEFT: &'static str = "PitOptRepairLeft";
    pub const PIT_REPAIR_LEFT: &'static str = "PitRepairLeft";
    pub const PITS_OPEN: &'static str = "PitsOpen";
    pub const PITSTOP_ACTIVE: &'static str = "PitstopActive";
    pub const PIT_SV_FLAGS: &'static str = "PitSvFlags";
    pub const PIT_SV_FUEL: &'static str = "PitSvFuel";
    pub const PIT_SV_LFP: &'static str = "PitSvLFP";
    pub const PIT_SV_LRP: &'static str = "PitSvLRP";
    pub const PIT_SV_RFP: &'static str = "PitSvRFP";
    pub const PIT_SV_RRP: &'static str = "PitSvRRP";
    pub const PIT_SV_TIRE_COMPOUND: &'static str = "PitSvTireCompound";

    /* Player Car */
    pub const PLAYER_CAR_CLASS: &'static str = "PlayerCarClass";
    pub const PLAYER_CAR_CLASS_POSITION: &'static str = "PlayerCarClassPosition";
    pub const PLAYER_CAR_DRIVER_INCIDENT_COUNT: &'static str = "PlayerCarDriverIncidentCount";
    pub const PLAYER_CAR_DRY_TIRE_SET_LIMIT: &'static str = "PlayerCarDryTireSetLimit";
    pub const PLAYER_CAR_IDX: &'static str = "PlayerCarIdx";
    pub const PLAYER_CAR_IN_PIT_STALL: &'static str = "PlayerCarInPitStall";
    pub const PLAYER_CAR_MY_INCIDENT_COUNT: &'static str = "PlayerCarMyIncidentCount";
    pub const PLAYER_CAR_PIT_SV_STATUS: &'static str = "PlayerCarPitSvStatus";
    pub const PLAYER_CAR_POSITION: &'static str = "PlayerCarPosition";
    pub const PLAYER_CAR_POWER_ADJUST: &'static str = "PlayerCarPowerAdjust";
    pub const PLAYER_CAR_SL_BLINK_RPM: &'static str = "PlayerCarSLBlinkRPM";
    pub const PLAYER_CAR_SL_FIRST_RPM: &'static str = "PlayerCarSLFirstRPM";
    pub const PLAYER_CAR_SL_LAST_RPM: &'static str = "PlayerCarSLLastRPM";
    pub const PLAYER_CAR_SL_SHIFT_RPM: &'static str = "PlayerCarSLShiftRPM";
    pub const PLAYER_CAR_TEAM_INCIDENT_COUNT: &'static str = "PlayerCarTeamIncidentCount";
    pub const PLAYER_CAR_TOW_TIME: &'static str = "PlayerCarTowTime";
    pub const PLAYER_CAR_WEIGHT_PENALTY: &'static str = "PlayerCarWeightPenalty";
    pub const PLAYER_FAST_REPAIRS_USED: &'static str = "PlayerFastRepairsUsed";
    pub const PLAYER_INCIDENTS: &'static str = "PlayerIncidents";
    pub const PLAYER_TIRE_COMPOUND: &'static str = "PlayerTireCompound";
    pub const PLAYER_TRACK_SURFACE: &'static str = "PlayerTrackSurface";
    pub const PLAYER_TRACK_SURFACE_MATERIAL: &'static str = "PlayerTrackSurfaceMaterial";

    /* Race */
    pub const RACE_LAPS: &'static str = "RaceLaps";

    /* Radio */
    pub const RADIO_TRANSMIT_CAR_IDX: &'static str = "RadioTransmitCarIdx";
    pub const RADIO_TRANSMIT_FREQUENCY_IDX: &'static str = "RadioTransmitFrequencyIdx";
    pub const RADIO_TRANSMIT_RADIO_IDX: &'static str = "RadioTransmitRadioIdx";

    /* Replay */
    pub const REPLAY_FRAME_NUM: &'static str = "ReplayFrameNum";
    pub const REPLAY_FRAME_NUM_END: &'static str = "ReplayFrameNumEnd";
    pub const REPLAY_PLAY_SLOW_MOTION: &'static str = "ReplayPlaySlowMotion";
    pub const REPLAY_PLAY_SPEED: &'static str = "ReplayPlaySpeed";
    pub const REPLAY_SESSION_NUM: &'static str = "ReplaySessionNum";
    pub const REPLAY_SESSION_TIME: &'static str = "ReplaySessionTime";

    /* Session */
    pub const SESSION_FLAGS: &'static str = "SessionFlags";
    pub const SESSION_JOKER_LAPS_REMAIN: &'static str = "SessionJokerLapsRemain";
    pub const SESSION_LAPS_REMAIN: &'static str = "SessionLapsRemain";
    pub const SESSION_LAPS_REMAIN_EX: &'static str = "SessionLapsRemainEx";
    pub const SESSION_LAPS_TOTAL: &'static str = "SessionLapsTotal";
    pub const SESSION_NUM: &'static str = "SessionNum";
    pub const SESSION_ON_JOKER_LAP: &'static str = "SessionOnJokerLap";
    pub const SESSION_STATE: &'static str = "SessionState";
    pub const SESSION_TICK: &'static str = "SessionTick";
    pub const SESSION_TIME: &'static str = "SessionTime";
    pub const SESSION_TIME_OF_DAY: &'static str = "SessionTimeOfDay";
    pub const SESSION_TIME_REMAIN: &'static str = "SessionTimeRemain";
    pub const SESSION_TIME_TOTAL: &'static str = "SessionTimeTotal";
    pub const SESSION_UNIQUE_ID: &'static str = "SessionUniqueID";

    /* Shifter */
    pub const SHIFTER: &'static str = "Shifter";

    /* Vehicle Dynamics */
    pub const SPEED: &'static str = "Speed";
    pub const VELOCITY_X: &'static str = "VelocityX";
    pub const VELOCITY_X_ST: &'static str = "VelocityX_ST";
    pub const VELOCITY_Y: &'static str = "VelocityY";
    pub const VELOCITY_Y_ST: &'static str = "VelocityY_ST";
    pub const VELOCITY_Z: &'static str = "VelocityZ";
    pub const VELOCITY_Z_ST: &'static str = "VelocityZ_ST";

    /* Steering */
    pub const STEERING_FFB_ENABLED: &'static str = "SteeringFFBEnabled";
    pub const STEERING_WHEEL_ANGLE: &'static str = "SteeringWheelAngle";
    pub const STEERING_WHEEL_ANGLE_MAX: &'static str = "SteeringWheelAngleMax";
    pub const STEERING_WHEEL_LIMITER: &'static str = "SteeringWheelLimiter";
    pub const STEERING_WHEEL_MAX_FORCE_NM: &'static str = "SteeringWheelMaxForceNm";
    pub const STEERING_WHEEL_PCT_DAMPER: &'static str = "SteeringWheelPctDamper";
    pub const STEERING_WHEEL_PCT_INTENSITY: &'static str = "SteeringWheelPctIntensity";
    pub const STEERING_WHEEL_PCT_SMOOTHING: &'static str = "SteeringWheelPctSmoothing";
    pub const STEERING_WHEEL_PCT_TORQUE: &'static str = "SteeringWheelPctTorque";
    pub const STEERING_WHEEL_PCT_TORQUE_SIGN: &'static str = "SteeringWheelPctTorqueSign";
    pub const STEERING_WHEEL_PCT_TORQUE_SIGN_STOPS: &'static str =
        "SteeringWheelPctTorqueSignStops";
    pub const STEERING_WHEEL_PEAK_FORCE_NM: &'static str = "SteeringWheelPeakForceNm";
    pub const STEERING_WHEEL_TORQUE: &'static str = "SteeringWheelTorque";
    pub const STEERING_WHEEL_TORQUE_ST: &'static str = "SteeringWheelTorque_ST";
    pub const STEERING_WHEEL_USE_LINEAR: &'static str = "SteeringWheelUseLinear";

    /* Video Capture */
    pub const VID_CAP_ACTIVE: &'static str = "VidCapActive";
    pub const VID_CAP_ENABLED: &'static str = "VidCapEnabled";
}
