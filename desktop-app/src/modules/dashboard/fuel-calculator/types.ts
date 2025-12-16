import { z } from "zod";

export const FuelCalculatorSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type FuelCalculatorSettings = z.infer<
  typeof FuelCalculatorSettingsSchema
>;
