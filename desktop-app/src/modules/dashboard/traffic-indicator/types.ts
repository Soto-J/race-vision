import { z } from "zod";

export const TrafficIndicatorSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TrafficIndicatorSettings = z.infer<
  typeof TrafficIndicatorSettingsSchema
>;
