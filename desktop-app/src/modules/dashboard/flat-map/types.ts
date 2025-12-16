import z from "zod";

export const FlatMapSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type FlatMapSettings = z.infer<typeof FlatMapSettingsSchema>;
