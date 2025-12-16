import { z } from "zod";

import {
  ConetentSchama,
  FooterSchema,
  GeneralSchema,
  HeaderSchema,
} from "@/modules/dashboard/types";

export const InputsSettingsSchema = z.object({
  isActive: z.boolean(),
  general: GeneralSchema,
  content: ConetentSchama,
  header: HeaderSchema,
  footer: FooterSchema,
});

export type InputsSettings = z.infer<typeof InputsSettingsSchema>;
