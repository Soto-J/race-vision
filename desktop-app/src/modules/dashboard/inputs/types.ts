import { z } from "zod";

export const InputsSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type InputsSettings = z.infer<typeof InputsSettingsSchema>;
