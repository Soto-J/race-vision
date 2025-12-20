import { z } from "zod";
import { InputsSettingsSchema } from "./inputs/types";

export const AppSettingsSchema = z.object({
  inputs: InputsSettingsSchema,
});

export type AppSettings = z.infer<typeof AppSettingsSchema>;
