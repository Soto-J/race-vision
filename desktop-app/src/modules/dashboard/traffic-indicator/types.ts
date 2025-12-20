import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const TrafficIndicatorSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TrafficIndicatorSettings = z.infer<
  typeof TrafficIndicatorSettingsSchema
>;
