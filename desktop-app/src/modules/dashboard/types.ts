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

export const DefaultBoolean = z.boolean().default(false);

export const GeneralSettingsSchema = z.object({
  useMetricSystem: DefaultBoolean,
  startupOverlayMinimized: DefaultBoolean,
  minimizeToSystemTray: DefaultBoolean,
  showRaceControlAtStartup: DefaultBoolean,
  useCTRLF6InsteadOfF6ToShow: DefaultBoolean,
  useHardwareAcceleration: DefaultBoolean,
  lowerFps: DefaultBoolean,
  showOverlaysInTaskbar: DefaultBoolean,
  VRCompatibility: DefaultBoolean,
  noVRCompatibility: DefaultBoolean,
});

export const AppSettingsSchema = z.object({
  general: GeneralSettingsSchema,
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

export type AppSettings = z.infer<typeof AppSettingsSchema>;

export const DEFAULT_SETTINGS = AppSettingsSchema.parse({});

// ****** Schemas shared with folder ************
export const GeneralSchema = z.object({
  showOverlayWhen: z.object({
    inCar: DefaultBoolean,
    outOfCar: DefaultBoolean,
    spotting: DefaultBoolean,
    inGarage: DefaultBoolean,
  }),
  showFlags: z.boolean().default(false),
});

export const ActiveAndDisplayInSchema = z.object({
  isActive: DefaultBoolean,
  displayIn: z.object({
    race: DefaultBoolean,
    qualy: DefaultBoolean,
    practice: DefaultBoolean,
  }),
});

export const PAGE_SETTINGS = {
  general: {
    schema: GeneralSettingsSchema,
    defaults: DEFAULT_SETTINGS.general,
  },
  standings: {
    schema: StandingsSettingsSchema,
    defaults: DEFAULT_SETTINGS.standings,
  },
  relative: {
    schema: RelativeSettingsSchema,
    defaults: DEFAULT_SETTINGS.relative,
  },
  inputs: {
    schema: InputsSettingsSchema,
    defaults: DEFAULT_SETTINGS.inputs,
  },
  inputsGraph: {
    schema: InputsGraphSettingsSchema,
    defaults: DEFAULT_SETTINGS.inputsGraph,
  },
  trafficIndicator: {
    schema: TrafficIndicatorSettingsSchema,
    defaults: DEFAULT_SETTINGS.trafficIndicator,
  },
  flatMap: {
    schema: FlatMapSettingsSchema,
    defaults: DEFAULT_SETTINGS.flatMap,
  },
  deltaBar: {
    schema: DeltaBarSettingsSchema,
    defaults: DEFAULT_SETTINGS.deltaBar,
  },
  trackMap: {
    schema: TrackMapSettingsSchema,
    defaults: DEFAULT_SETTINGS.trackMap,
  },
  twitchChat: {
    schema: TwitchChatSettingsSchema,
    defaults: DEFAULT_SETTINGS.twitchChat,
  },
} as const;
