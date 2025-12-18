import { z } from "zod";
import { DefaultBoolean } from "../types";

export const FlatMapSettingsSchema = z.object({
  isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type FlatMapSettings = z.infer<typeof FlatMapSettingsSchema>;
