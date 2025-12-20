import { z } from "zod";
import { DefaultBoolean } from "../common-schemas";

export const StandingsSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type StandingsSettings = z.infer<typeof StandingsSettingsSchema>;
