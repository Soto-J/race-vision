import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const PitHelperSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type PitHelperSettings = z.infer<typeof PitHelperSettingsSchema>;
