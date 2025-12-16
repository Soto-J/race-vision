import { z } from "zod";

export const InputsGraphSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type InputsGraphSettings = z.infer<typeof InputsGraphSettingsSchema>;
