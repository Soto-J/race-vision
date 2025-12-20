import { z } from "zod";
import { DefaultBoolean } from "../types";

export const TrafficIndicatorSettingsSchema = z.object({
  isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TrafficIndicatorSettings = z.infer<
  typeof TrafficIndicatorSettingsSchema
>;
