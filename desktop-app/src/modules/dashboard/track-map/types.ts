import { z } from "zod";

export const TrackMapSettingsSchema = z.object({
  isActive: z.boolean().default(false),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TrackMapSettings = z.infer<typeof TrackMapSettingsSchema>;
