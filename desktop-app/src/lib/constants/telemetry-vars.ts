export const TelemetryVars = {
  /* Environmental */
  AIR_DENSITY: "AirDensity",
  AIR_PRESSURE: "AirPressure",
  AIR_TEMP: "AirTemp",
  FOG_LEVEL: "FogLevel",
  PRECIPITATION: "Precipitation",
  RELATIVE_HUMIDITY: "RelativeHumidity",
  SKIES: "Skies",
  SOLAR_ALTITUDE: "SolarAltitude",
  SOLAR_AZIMUTH: "SolarAzimuth",
  TRACK_TEMP: "TrackTemp",
  TRACK_TEMP_CREW: "TrackTempCrew",
  TRACK_WETNESS: "TrackWetness",
  WEATHER_DECLARED_WET: "WeatherDeclaredWet",
  WIND_DIR: "WindDir",
  WIND_VEL: "WindVel",

  /* Pedals */
  BRAKE: "Brake",
  BRAKE_ABS_ACTIVE: "BrakeABSactive",
  BRAKE_RAW: "BrakeRaw",
  CLUTCH: "Clutch",
  CLUTCH_RAW: "ClutchRaw",
  HANDBRAKE_RAW: "HandbrakeRaw",
  THROTTLE: "Throttle",
  THROTTLE_RAW: "ThrottleRaw",

  /* Camera */
  CAM_CAMERA_NUMBER: "CamCameraNumber",
  CAM_CAMERA_STATE: "CamCameraState",
  CAM_CAR_IDX: "CamCarIdx",
  CAM_GROUP_NUMBER: "CamGroupNumber",

  /* Car Position/Distance */
  CAR_DIST_AHEAD: "CarDistAhead",
  CAR_DIST_BEHIND: "CarDistBehind",
  CAR_LEFT_RIGHT: "CarLeftRight",

  /* Car Index Arrays - Best Lap */
  CAR_IDX_BEST_LAP_NUM: "CarIdxBestLapNum",
  CAR_IDX_BEST_LAP_TIME: "CarIdxBestLapTime",

  /* Car Index Arrays - Class/Position */
  CAR_IDX_CLASS: "CarIdxClass",
  CAR_IDX_CLASS_POSITION: "CarIdxClassPosition",
  CAR_IDX_POSITION: "CarIdxPosition",

  /* Car Index Arrays - Timing */
  CAR_IDX_EST_TIME: "CarIdxEstTime",
  CAR_IDX_F2_TIME: "CarIdxF2Time",
  CAR_IDX_LAST_LAP_TIME: "CarIdxLastLapTime",

  /* Car Index Arrays - Repairs */
  CAR_IDX_FAST_REPAIRS_USED: "CarIdxFastRepairsUsed",

  /* Car Index Arrays - Gear/RPM */
  CAR_IDX_GEAR: "CarIdxGear",
  CAR_IDX_RPM: "CarIdxRPM",

  /* Car Index Arrays - Laps */
  CAR_IDX_LAP: "CarIdxLap",
  CAR_IDX_LAP_COMPLETED: "CarIdxLapCompleted",
  CAR_IDX_LAP_DIST_PCT: "CarIdxLapDistPct",

  /* Car Index Arrays - Pit/Track */
  CAR_IDX_ON_PIT_ROAD: "CarIdxOnPitRoad",
  CAR_IDX_TRACK_SURFACE: "CarIdxTrackSurface",
  CAR_IDX_TRACK_SURFACE_MATERIAL: "CarIdxTrackSurfaceMaterial",

  /* Car Index Arrays - Push to Pass */
  CAR_IDX_P2P_COUNT: "CarIdxP2P_Count",
  CAR_IDX_P2P_STATUS: "CarIdxP2P_Status",

  /* Car Index Arrays - Pacing */
  CAR_IDX_PACE_FLAGS: "CarIdxPaceFlags",
  CAR_IDX_PACE_LINE: "CarIdxPaceLine",
  CAR_IDX_PACE_ROW: "CarIdxPaceRow",

  /* Car Index Arrays - Tires */
  CAR_IDX_QUAL_TIRE_COMPOUND: "CarIdxQualTireCompound",
  CAR_IDX_QUAL_TIRE_COMPOUND_LOCKED: "CarIdxQualTireCompoundLocked",
  CAR_IDX_TIRE_COMPOUND: "CarIdxTireCompound",

  /* Car Index Arrays - Flags/Steering */
  CAR_IDX_SESSION_FLAGS: "CarIdxSessionFlags",
  CAR_IDX_STEER: "CarIdxSteer",

  /* Communications */
  CHAN_AVG_LATENCY: "ChanAvgLatency",
  CHAN_CLOCK_SKEW: "ChanClockSkew",
  CHAN_LATENCY: "ChanLatency",
  CHAN_PARTNER_QUALITY: "ChanPartnerQuality",
  CHAN_QUALITY: "ChanQuality",

  /* Performance Monitoring */
  CPU_USAGE_BG: "CpuUsageBG",
  CPU_USAGE_FG: "CpuUsageFG",
  FRAME_RATE: "FrameRate",
  GPU_USAGE: "GpuUsage",
  MEM_PAGE_FAULT_SEC: "MemPageFaultSec",
  MEM_SOFT_PAGE_FAULT_SEC: "MemSoftPageFaultSec",

  /* Driver Change */
  DC_DRIVERS_SO_FAR: "DCDriversSoFar",
  DC_LAP_STATUS: "DCLapStatus",

  /* In-Car Controls */
  DC_PIT_SPEED_LIMITER_TOGGLE: "dcPitSpeedLimiterToggle",
  DC_STARTER: "dcStarter",
  DC_TOGGLE_WINDSHIELD_WIPERS: "dcToggleWindshieldWipers",
  DC_TRIGGER_WINDSHIELD_WIPERS: "dcTriggerWindshieldWipers",

  /* Settings */
  DISPLAY_UNITS: "DisplayUnits",

  /* Pitstop Settings */
  DP_FAST_REPAIR: "dpFastRepair",
  DP_FUEL_ADD_KG: "dpFuelAddKg",
  DP_FUEL_AUTO_FILL_ACTIVE: "dpFuelAutoFillActive",
  DP_FUEL_AUTO_FILL_ENABLED: "dpFuelAutoFillEnabled",
  DP_FUEL_FILL: "dpFuelFill",
  DP_LF_TIRE_CHANGE: "dpLFTireChange",
  DP_LF_TIRE_COLD_PRESS: "dpLFTireColdPress",
  DP_LR_TIRE_CHANGE: "dpLRTireChange",
  DP_LR_TIRE_COLD_PRESS: "dpLRTireColdPress",
  DP_RF_TIRE_CHANGE: "dpRFTireChange",
  DP_RF_TIRE_COLD_PRESS: "dpRFTireColdPress",
  DP_RR_TIRE_CHANGE: "dpRRTireChange",
  DP_RR_TIRE_COLD_PRESS: "dpRRTireColdPress",
  DP_WINDSHIELD_TEAROFF: "dpWindshieldTearoff",

  /* Driver/Car State */
  DRIVER_MARKER: "DriverMarker",

  /* Engine */
  ENGINE0_RPM: "Engine0_RPM",
  ENGINE_WARNINGS: "EngineWarnings",
  FUEL_LEVEL: "FuelLevel",
  FUEL_LEVEL_PCT: "FuelLevelPct",
  FUEL_PRESS: "FuelPress",
  FUEL_USE_PER_HOUR: "FuelUsePerHour",
  GEAR: "Gear",
  MANIFOLD_PRESS: "ManifoldPress",
  OIL_LEVEL: "OilLevel",
  OIL_PRESS: "OilPress",
  OIL_TEMP: "OilTemp",
  RPM: "RPM",
  SHIFT_GRIND_RPM: "ShiftGrindRPM",
  SHIFT_INDICATOR_PCT: "ShiftIndicatorPct",
  SHIFT_POWER_PCT: "ShiftPowerPct",
  VOLTAGE: "Voltage",
  WATER_LEVEL: "WaterLevel",
  WATER_TEMP: "WaterTemp",

  /* Hybrid */
  MANUAL_BOOST: "ManualBoost",
  MANUAL_NO_BOOST: "ManualNoBoost",

  /* Session Control */
  ENTER_EXIT_RESET: "EnterExitReset",

  /* Repairs */
  FAST_REPAIR_AVAILABLE: "FastRepairAvailable",
  FAST_REPAIR_USED: "FastRepairUsed",

  /* Tire Sets */
  FRONT_TIRE_SETS_AVAILABLE: "FrontTireSetsAvailable",
  FRONT_TIRE_SETS_USED: "FrontTireSetsUsed",
  LEFT_TIRE_SETS_AVAILABLE: "LeftTireSetsAvailable",
  LEFT_TIRE_SETS_USED: "LeftTireSetsUsed",
  LF_TIRES_AVAILABLE: "LFTiresAvailable",
  LF_TIRES_USED: "LFTiresUsed",
  LR_TIRES_AVAILABLE: "LRTiresAvailable",
  LR_TIRES_USED: "LRTiresUsed",
  REAR_TIRE_SETS_AVAILABLE: "RearTireSetsAvailable",
  REAR_TIRE_SETS_USED: "RearTireSetsUsed",
  RF_TIRES_AVAILABLE: "RFTiresAvailable",
  RF_TIRES_USED: "RFTiresUsed",
  RIGHT_TIRE_SETS_AVAILABLE: "RightTireSetsAvailable",
  RIGHT_TIRE_SETS_USED: "RightTireSetsUsed",
  RR_TIRES_AVAILABLE: "RRTiresAvailable",
  RR_TIRES_USED: "RRTiresUsed",
  TIRE_SETS_AVAILABLE: "TireSetsAvailable",
  TIRE_SETS_USED: "TireSetsUsed",

  /* Telemetry System */
  IS_DISK_LOGGING_ACTIVE: "IsDiskLoggingActive",
  IS_DISK_LOGGING_ENABLED: "IsDiskLoggingEnabled",

  /* Garage/Track State */
  IS_GARAGE_VISIBLE: "IsGarageVisible",
  IS_IN_GARAGE: "IsInGarage",
  IS_ON_TRACK: "IsOnTrack",
  IS_ON_TRACK_CAR: "IsOnTrackCar",
  IS_REPLAY_PLAYING: "IsReplayPlaying",

  /* Lap Timing */
  LAP: "Lap",
  LAP_BEST_LAP: "LapBestLap",
  LAP_BEST_LAP_TIME: "LapBestLapTime",
  LAP_BEST_N_LAP_LAP: "LapBestNLapLap",
  LAP_BEST_N_LAP_TIME: "LapBestNLapTime",
  LAP_COMPLETED: "LapCompleted",
  LAP_CURRENT_LAP_TIME: "LapCurrentLapTime",
  LAP_DELTA_TO_BEST_LAP: "LapDeltaToBestLap",
  LAP_DELTA_TO_BEST_LAP_DD: "LapDeltaToBestLap_DD",
  LAP_DELTA_TO_BEST_LAP_OK: "LapDeltaToBestLap_OK",
  LAP_DELTA_TO_OPTIMAL_LAP: "LapDeltaToOptimalLap",
  LAP_DELTA_TO_OPTIMAL_LAP_DD: "LapDeltaToOptimalLap_DD",
  LAP_DELTA_TO_OPTIMAL_LAP_OK: "LapDeltaToOptimalLap_OK",
  LAP_DELTA_TO_SESSION_BEST_LAP: "LapDeltaToSessionBestLap",
  LAP_DELTA_TO_SESSION_BEST_LAP_DD: "LapDeltaToSessionBestLap_DD",
  LAP_DELTA_TO_SESSION_BEST_LAP_OK: "LapDeltaToSessionBestLap_OK",
  LAP_DELTA_TO_SESSION_LASTL_LAP: "LapDeltaToSessionLastlLap",
  LAP_DELTA_TO_SESSION_LASTL_LAP_DD: "LapDeltaToSessionLastlLap_DD",
  LAP_DELTA_TO_SESSION_LASTL_LAP_OK: "LapDeltaToSessionLastlLap_OK",
  LAP_DELTA_TO_SESSION_OPTIMAL_LAP: "LapDeltaToSessionOptimalLap",
  LAP_DELTA_TO_SESSION_OPTIMAL_LAP_DD: "LapDeltaToSessionOptimalLap_DD",
  LAP_DELTA_TO_SESSION_OPTIMAL_LAP_OK: "LapDeltaToSessionOptimalLap_OK",
  LAP_DIST: "LapDist",
  LAP_DIST_PCT: "LapDistPct",
  LAP_LAS_N_LAP_SEQ: "LapLasNLapSeq",
  LAP_LAST_LAP_TIME: "LapLastLapTime",
  LAP_LAST_N_LAP_TIME: "LapLastNLapTime",

  /* Acceleration */
  LAT_ACCEL: "LatAccel",
  LAT_ACCEL_ST: "LatAccel_ST",
  LONG_ACCEL: "LongAccel",
  LONG_ACCEL_ST: "LongAccel_ST",
  VERT_ACCEL: "VertAccel",
  VERT_ACCEL_ST: "VertAccel_ST",

  /* Left Front Tire */
  LF_BRAKE_LINE_PRESS: "LFbrakeLinePress",
  LF_COLD_PRESSURE: "LFcoldPressure",
  LF_ODOMETER: "LFodometer",
  LF_SHOCK_DEFL: "LFshockDefl",
  LF_SHOCK_DEFL_ST: "LFshockDefl_ST",
  LF_SHOCK_VEL: "LFshockVel",
  LF_SHOCK_VEL_ST: "LFshockVel_ST",
  LF_TEMP_CL: "LFtempCL",
  LF_TEMP_CM: "LFtempCM",
  LF_TEMP_CR: "LFtempCR",
  LF_WEAR_L: "LFwearL",
  LF_WEAR_M: "LFwearM",
  LF_WEAR_R: "LFwearR",

  /* Left Rear Tire */
  LR_BRAKE_LINE_PRESS: "LRbrakeLinePress",
  LR_COLD_PRESSURE: "LRcoldPressure",
  LR_ODOMETER: "LRodometer",
  LR_SHOCK_DEFL: "LRshockDefl",
  LR_SHOCK_DEFL_ST: "LRshockDefl_ST",
  LR_SHOCK_VEL: "LRshockVel",
  LR_SHOCK_VEL_ST: "LRshockVel_ST",
  LR_TEMP_CL: "LRtempCL",
  LR_TEMP_CM: "LRtempCM",
  LR_TEMP_CR: "LRtempCR",
  LR_WEAR_L: "LRwearL",
  LR_WEAR_M: "LRwearM",
  LR_WEAR_R: "LRwearR",

  /* Right Front Tire */
  RF_BRAKE_LINE_PRESS: "RFbrakeLinePress",
  RF_COLD_PRESSURE: "RFcoldPressure",
  RF_ODOMETER: "RFodometer",
  RF_SHOCK_DEFL: "RFshockDefl",
  RF_SHOCK_DEFL_ST: "RFshockDefl_ST",
  RF_SHOCK_VEL: "RFshockVel",
  RF_SHOCK_VEL_ST: "RFshockVel_ST",
  RF_TEMP_CL: "RFtempCL",
  RF_TEMP_CM: "RFtempCM",
  RF_TEMP_CR: "RFtempCR",
  RF_WEAR_L: "RFwearL",
  RF_WEAR_M: "RFwearM",
  RF_WEAR_R: "RFwearR",

  /* Right Rear Tire */
  RR_BRAKE_LINE_PRESS: "RRbrakeLinePress",
  RR_COLD_PRESSURE: "RRcoldPressure",
  RR_ODOMETER: "RRodometer",
  RR_SHOCK_DEFL: "RRshockDefl",
  RR_SHOCK_DEFL_ST: "RRshockDefl_ST",
  RR_SHOCK_VEL: "RRshockVel",
  RR_SHOCK_VEL_ST: "RRshockVel_ST",
  RR_TEMP_CL: "RRtempCL",
  RR_TEMP_CM: "RRtempCM",
  RR_TEMP_CR: "RRtempCR",
  RR_WEAR_L: "RRwearL",
  RR_WEAR_M: "RRwearM",
  RR_WEAR_R: "RRwearR",

  /* Tire Rumble */
  TIRE_LF_RUMBLE_PITCH: "TireLF_RumblePitch",
  TIRE_LR_RUMBLE_PITCH: "TireLR_RumblePitch",
  TIRE_RF_RUMBLE_PITCH: "TireRF_RumblePitch",
  TIRE_RR_RUMBLE_PITCH: "TireRR_RumblePitch",

  /* Textures */
  LOAD_NUM_TEXTURES: "LoadNumTextures",
  OK_TO_RELOAD_TEXTURES: "OkToReloadTextures",

  /* Pit Road */
  ON_PIT_ROAD: "OnPitRoad",

  /* Push to Pass */
  P2P_COUNT: "P2P_Count",
  P2P_STATUS: "P2P_Status",
  PUSH_TO_PASS: "PushToPass",
  PUSH_TO_TALK: "PushToTalk",

  /* Pace Mode */
  PACE_MODE: "PaceMode",

  /* Orientation */
  PITCH: "Pitch",
  PITCH_RATE: "PitchRate",
  PITCH_RATE_ST: "PitchRate_ST",
  ROLL: "Roll",
  ROLL_RATE: "RollRate",
  ROLL_RATE_ST: "RollRate_ST",
  YAW: "Yaw",
  YAW_NORTH: "YawNorth",
  YAW_RATE: "YawRate",
  YAW_RATE_ST: "YawRate_ST",

  /* Pit Service */
  PIT_OPT_REPAIR_LEFT: "PitOptRepairLeft",
  PIT_REPAIR_LEFT: "PitRepairLeft",
  PITS_OPEN: "PitsOpen",
  PITSTOP_ACTIVE: "PitstopActive",
  PIT_SV_FLAGS: "PitSvFlags",
  PIT_SV_FUEL: "PitSvFuel",
  PIT_SV_LFP: "PitSvLFP",
  PIT_SV_LRP: "PitSvLRP",
  PIT_SV_RFP: "PitSvRFP",
  PIT_SV_RRP: "PitSvRRP",
  PIT_SV_TIRE_COMPOUND: "PitSvTireCompound",

  /* Player Car */
  PLAYER_CAR_CLASS: "PlayerCarClass",
  PLAYER_CAR_CLASS_POSITION: "PlayerCarClassPosition",
  PLAYER_CAR_DRIVER_INCIDENT_COUNT: "PlayerCarDriverIncidentCount",
  PLAYER_CAR_DRY_TIRE_SET_LIMIT: "PlayerCarDryTireSetLimit",
  PLAYER_CAR_IDX: "PlayerCarIdx",
  PLAYER_CAR_IN_PIT_STALL: "PlayerCarInPitStall",
  PLAYER_CAR_MY_INCIDENT_COUNT: "PlayerCarMyIncidentCount",
  PLAYER_CAR_PIT_SV_STATUS: "PlayerCarPitSvStatus",
  PLAYER_CAR_POSITION: "PlayerCarPosition",
  PLAYER_CAR_POWER_ADJUST: "PlayerCarPowerAdjust",
  PLAYER_CAR_SL_BLINK_RPM: "PlayerCarSLBlinkRPM",
  PLAYER_CAR_SL_FIRST_RPM: "PlayerCarSLFirstRPM",
  PLAYER_CAR_SL_LAST_RPM: "PlayerCarSLLastRPM",
  PLAYER_CAR_SL_SHIFT_RPM: "PlayerCarSLShiftRPM",
  PLAYER_CAR_TEAM_INCIDENT_COUNT: "PlayerCarTeamIncidentCount",
  PLAYER_CAR_TOW_TIME: "PlayerCarTowTime",
  PLAYER_CAR_WEIGHT_PENALTY: "PlayerCarWeightPenalty",
  PLAYER_FAST_REPAIRS_USED: "PlayerFastRepairsUsed",
  PLAYER_INCIDENTS: "PlayerIncidents",
  PLAYER_TIRE_COMPOUND: "PlayerTireCompound",
  PLAYER_TRACK_SURFACE: "PlayerTrackSurface",
  PLAYER_TRACK_SURFACE_MATERIAL: "PlayerTrackSurfaceMaterial",

  /* Race */
  RACE_LAPS: "RaceLaps",

  /* Radio */
  RADIO_TRANSMIT_CAR_IDX: "RadioTransmitCarIdx",
  RADIO_TRANSMIT_FREQUENCY_IDX: "RadioTransmitFrequencyIdx",
  RADIO_TRANSMIT_RADIO_IDX: "RadioTransmitRadioIdx",

  /* Replay */
  REPLAY_FRAME_NUM: "ReplayFrameNum",
  REPLAY_FRAME_NUM_END: "ReplayFrameNumEnd",
  REPLAY_PLAY_SLOW_MOTION: "ReplayPlaySlowMotion",
  REPLAY_PLAY_SPEED: "ReplayPlaySpeed",
  REPLAY_SESSION_NUM: "ReplaySessionNum",
  REPLAY_SESSION_TIME: "ReplaySessionTime",

  /* Session */
  SESSION_FLAGS: "SessionFlags",
  SESSION_JOKER_LAPS_REMAIN: "SessionJokerLapsRemain",
  SESSION_LAPS_REMAIN: "SessionLapsRemain",
  SESSION_LAPS_REMAIN_EX: "SessionLapsRemainEx",
  SESSION_LAPS_TOTAL: "SessionLapsTotal",
  SESSION_NUM: "SessionNum",
  SESSION_ON_JOKER_LAP: "SessionOnJokerLap",
  SESSION_STATE: "SessionState",
  SESSION_TICK: "SessionTick",
  SESSION_TIME: "SessionTime",
  SESSION_TIME_OF_DAY: "SessionTimeOfDay",
  SESSION_TIME_REMAIN: "SessionTimeRemain",
  SESSION_TIME_TOTAL: "SessionTimeTotal",
  SESSION_UNIQUE_ID: "SessionUniqueID",

  /* Shifter */
  SHIFTER: "Shifter",

  /* Vehicle Dynamics */
  SPEED: "Speed",
  VELOCITY_X: "VelocityX",
  VELOCITY_X_ST: "VelocityX_ST",
  VELOCITY_Y: "VelocityY",
  VELOCITY_Y_ST: "VelocityY_ST",
  VELOCITY_Z: "VelocityZ",
  VELOCITY_Z_ST: "VelocityZ_ST",

  /* Steering */
  STEERING_FFB_ENABLED: "SteeringFFBEnabled",
  STEERING_WHEEL_ANGLE: "SteeringWheelAngle",
  STEERING_WHEEL_ANGLE_MAX: "SteeringWheelAngleMax",
  STEERING_WHEEL_LIMITER: "SteeringWheelLimiter",
  STEERING_WHEEL_MAX_FORCE_NM: "SteeringWheelMaxForceNm",
  STEERING_WHEEL_PCT_DAMPER: "SteeringWheelPctDamper",
  STEERING_WHEEL_PCT_INTENSITY: "SteeringWheelPctIntensity",
  STEERING_WHEEL_PCT_SMOOTHING: "SteeringWheelPctSmoothing",
  STEERING_WHEEL_PCT_TORQUE: "SteeringWheelPctTorque",
  STEERING_WHEEL_PCT_TORQUE_SIGN: "SteeringWheelPctTorqueSign",
  STEERING_WHEEL_PCT_TORQUE_SIGN_STOPS: "SteeringWheelPctTorqueSignStops",
  STEERING_WHEEL_PEAK_FORCE_NM: "SteeringWheelPeakForceNm",
  STEERING_WHEEL_TORQUE: "SteeringWheelTorque",
  STEERING_WHEEL_TORQUE_ST: "SteeringWheelTorque_ST",
  STEERING_WHEEL_USE_LINEAR: "SteeringWheelUseLinear",

  /* Video Capture */
  VID_CAP_ACTIVE: "VidCapActive",
  VID_CAP_ENABLED: "VidCapEnabled",
} as const;

export type TelemetryVar = (typeof TelemetryVars)[keyof typeof TelemetryVars];
