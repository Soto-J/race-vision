import { z } from "zod";

export const DeltaBarSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type DeltaBarSettings = z.infer<typeof DeltaBarSettingsSchema>;
