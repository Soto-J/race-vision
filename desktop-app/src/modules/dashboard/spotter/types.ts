import { z } from "zod";

export const SpotterSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type SpotterSettings = z.infer<typeof SpotterSettingsSchema>;
