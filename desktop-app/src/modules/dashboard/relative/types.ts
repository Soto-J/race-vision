import z from "zod";

export const RelativeSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type RelativeSettings = z.infer<typeof RelativeSettingsSchema>;
