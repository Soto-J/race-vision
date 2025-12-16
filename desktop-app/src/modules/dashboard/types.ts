import { z } from "zod";

import { InputsSettingsSchema } from "./inputs/types";
import { FuelCalculatorSettingsSchema } from "./fuel-calculator/types";
import { SpotterSettingsSchema } from "./spotter/types";
import { StandingsSettingsSchema } from "./standings/types";
import { PitHelperSettingsSchema } from "./pit-helper/types";
import { FlatMapSettingsSchema } from "./flat-map/types";
import { InputsGraphSettingsSchema } from "./inputs-graph/types";
import { TrafficIndicatorSettingsSchema } from "./traffic-indicator/types";
import { DeltaBarSettingsSchema } from "./delta-bar/types";
import { TrackMapSettingsSchema } from "./track-map/types";
import { TwitchChatSettingsSchema } from "./twitch-chat/types";
import { RelativeSettingsSchema } from "./relative/types";

const PageSettingsSchema = z.object({
  general: z.object({
    useMetricSystem: z.boolean(),
    startupOverlayMinimized: z.boolean(),
    minimizeToSystemTray: z.boolean(),
    showRaceControlAtStartup: z.boolean(),
    useCTRLF6InsteadOfF6ToShow: z.boolean(),
    useHardwareAcceleration: z.boolean(),
    lowerFps: z.boolean(),
    showOverlaysInTaskbar: z.boolean(),
    VRCompatibility: z.boolean(),
    noVRCompatibility: z.boolean(),
  }),
  standings: StandingsSettingsSchema,
  relative: RelativeSettingsSchema,
  fuelCalculator: FuelCalculatorSettingsSchema,
  spotter: SpotterSettingsSchema,
  pitHelper: PitHelperSettingsSchema,
  inputs: InputsSettingsSchema,
  inputsGraph: InputsGraphSettingsSchema,
  trafficIndicator: TrafficIndicatorSettingsSchema,
  flatMap: FlatMapSettingsSchema,
  deltaBar: DeltaBarSettingsSchema,
  trackMap: TrackMapSettingsSchema,
  twitchChat: TwitchChatSettingsSchema,
});

export type PageSettings = z.infer<typeof PageSettingsSchema>;

// *** Schemas shared with folder ************
export const GeneralSchema = z.object({
  showOverlayWhen: z.object({
    inCar: z.boolean(),
    outOfCar: z.boolean(),
    spotting: z.boolean(),
    inGarage: z.boolean(),
  }),
  showFlags: z.boolean().default(false),
});

export const SettingsSchema = z.object({
  isActive: z.boolean().default(false),
  displayIn: z.object({
    race: z.boolean().default(true),
    qualy: z.boolean().default(true),
    practice: z.boolean().default(true),
  }),
});

export const ConetentSchama = z.object({
  revLights: SettingsSchema,
  gearsAndSpeed: SettingsSchema,
  inputsGraph: SettingsSchema,
  ABSActivation: SettingsSchema,
  inputBars: SettingsSchema,
  boostERS: SettingsSchema,
  cornerSpeed: SettingsSchema,
});

export const HeaderSchema = z.object({
  sessionName: SettingsSchema,
  eventType: SettingsSchema,
  trackName: SettingsSchema,
  localtime24h: SettingsSchema,
  localtimeAmPm: SettingsSchema,
  inSimTime24h: SettingsSchema,
  inSimTimeAmPm: SettingsSchema,
  airTemp: SettingsSchema,
  trackTemp: SettingsSchema,
  humidity: SettingsSchema,
  fogLevel: SettingsSchema,
  timeRemaining: SettingsSchema,
  lapsRemaining: SettingsSchema,
  incidentCount: SettingsSchema,
  currentLapTime: SettingsSchema,
  sessionBestLapTime: SettingsSchema,
  lastLapTime: SettingsSchema,
  lastLapTimeCalculated: SettingsSchema,
  lapDeltaBest: SettingsSchema,
  lapDeltaOptimal: SettingsSchema,
  lapDeltaSessionBest: SettingsSchema,
  lapDeltaSessionOptimal: SettingsSchema,
  lapDeltaSessionLast: SettingsSchema,
  brakeBias: SettingsSchema,
  fuelLevel: SettingsSchema,
  waterTemp: SettingsSchema,
  oilTemp: SettingsSchema,
  sof: SettingsSchema,
  currentStintInLaps: SettingsSchema,
  currentStintInTime: SettingsSchema,
  rpm: SettingsSchema,
  deployMode: SettingsSchema,
  arbFront: SettingsSchema,
  arbRear: SettingsSchema,
  abs: SettingsSchema,
  tc1: SettingsSchema,
  tc2: SettingsSchema,
  weightJacker: SettingsSchema,
  rearBrakeValve: SettingsSchema,
  precipitation: SettingsSchema,
  trackWetness: SettingsSchema,
  weatherDeclaredWet: SettingsSchema,
  pitTimeLossBeta: SettingsSchema,
  windDirectionForDriver: SettingsSchema,
  predictedPositionAfterPitStop: SettingsSchema,
  iRatingAndGain: SettingsSchema,
  pushToPass: SettingsSchema,
});

export const FooterSchema = HeaderSchema;
