import { z } from "zod";

import {
  ActiveAndDisplayInSchema,
  DefaultBoolean,
  GeneralSchema,
} from "@/modules/dashboard/types";

export const ContentSchema = z.object({
  revLights: ActiveAndDisplayInSchema,
  gearsAndSpeed: ActiveAndDisplayInSchema,
  inputsGraph: ActiveAndDisplayInSchema,
  ABSActivation: ActiveAndDisplayInSchema,
  inputBars: ActiveAndDisplayInSchema,
  boostERS: ActiveAndDisplayInSchema,
  cornerSpeed: ActiveAndDisplayInSchema,
});

export const HeaderSchema = z.object({
  sessionName: ActiveAndDisplayInSchema,
  eventType: ActiveAndDisplayInSchema,
  trackName: ActiveAndDisplayInSchema,
  localtime24h: ActiveAndDisplayInSchema,
  localtimeAmPm: ActiveAndDisplayInSchema,
  inSimTime24h: ActiveAndDisplayInSchema,
  inSimTimeAmPm: ActiveAndDisplayInSchema,
  airTemp: ActiveAndDisplayInSchema,
  trackTemp: ActiveAndDisplayInSchema,
  humidity: ActiveAndDisplayInSchema,
  fogLevel: ActiveAndDisplayInSchema,
  timeRemaining: ActiveAndDisplayInSchema,
  lapsRemaining: ActiveAndDisplayInSchema,
  incidentCount: ActiveAndDisplayInSchema,
  currentLapTime: ActiveAndDisplayInSchema,
  sessionBestLapTime: ActiveAndDisplayInSchema,
  lastLapTime: ActiveAndDisplayInSchema,
  lastLapTimeCalculated: ActiveAndDisplayInSchema,
  lapDeltaBest: ActiveAndDisplayInSchema,
  lapDeltaOptimal: ActiveAndDisplayInSchema,
  lapDeltaSessionBest: ActiveAndDisplayInSchema,
  lapDeltaSessionOptimal: ActiveAndDisplayInSchema,
  lapDeltaSessionLast: ActiveAndDisplayInSchema,
  brakeBias: ActiveAndDisplayInSchema,
  fuelLevel: ActiveAndDisplayInSchema,
  waterTemp: ActiveAndDisplayInSchema,
  oilTemp: ActiveAndDisplayInSchema,
  sof: ActiveAndDisplayInSchema,
  currentStintInLaps: ActiveAndDisplayInSchema,
  currentStintInTime: ActiveAndDisplayInSchema,
  rpm: ActiveAndDisplayInSchema,
  deployMode: ActiveAndDisplayInSchema,
  arbFront: ActiveAndDisplayInSchema,
  arbRear: ActiveAndDisplayInSchema,
  abs: ActiveAndDisplayInSchema,
  tc1: ActiveAndDisplayInSchema,
  tc2: ActiveAndDisplayInSchema,
  weightJacker: ActiveAndDisplayInSchema,
  rearBrakeValve: ActiveAndDisplayInSchema,
  precipitation: ActiveAndDisplayInSchema,
  trackWetness: ActiveAndDisplayInSchema,
  weatherDeclaredWet: ActiveAndDisplayInSchema,
  pitTimeLossBeta: ActiveAndDisplayInSchema,
  windDirectionForDriver: ActiveAndDisplayInSchema,
  predictedPositionAfterPitStop: ActiveAndDisplayInSchema,
  iRatingAndGain: ActiveAndDisplayInSchema,
  pushToPass: ActiveAndDisplayInSchema,
});

export const FooterSchema = HeaderSchema;

export const InputsSettingsSchema = z.object({
  isActive: DefaultBoolean,
  general: GeneralSchema,
  content: ContentSchema,
  header: HeaderSchema,
  footer: FooterSchema,
});

export type InputsSettings = z.infer<typeof InputsSettingsSchema>;
