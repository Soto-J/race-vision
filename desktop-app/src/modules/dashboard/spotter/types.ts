import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const SpotterSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type SpotterSettings = z.infer<typeof SpotterSettingsSchema>;
