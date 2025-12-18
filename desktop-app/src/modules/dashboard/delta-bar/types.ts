import { z } from "zod";
import { DefaultBoolean } from "../types";

export const DeltaBarSettingsSchema = z.object({
  isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type DeltaBarSettings = z.infer<typeof DeltaBarSettingsSchema>;
