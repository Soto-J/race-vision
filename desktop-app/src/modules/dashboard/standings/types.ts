import { z } from "zod";

export const StandingsSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type StandingsSettings = z.infer<typeof StandingsSettingsSchema>;
