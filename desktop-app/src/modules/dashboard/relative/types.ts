import { z } from "zod";
import { DefaultBoolean } from "../types";

export const RelativeSettingsSchema = z.object({
  isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type RelativeSettings = z.infer<typeof RelativeSettingsSchema>;
