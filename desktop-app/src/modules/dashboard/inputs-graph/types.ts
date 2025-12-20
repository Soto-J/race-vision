import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const InputsGraphSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type InputsGraphSettings = z.infer<typeof InputsGraphSettingsSchema>;
