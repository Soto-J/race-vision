import { z } from "zod";

export const PitHelperSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type PitHelperSettings = z.infer<typeof PitHelperSettingsSchema>;
