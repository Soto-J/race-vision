import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const RelativeSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type RelativeSettings = z.infer<typeof RelativeSettingsSchema>;
