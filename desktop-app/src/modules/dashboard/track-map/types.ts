import z from "zod";

export const TrackMapSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TrackMapSettings = z.infer<typeof TrackMapSettingsSchema>;
