import { createSettingsHook } from ".";

import { InputsSettingsSchema } from "@/modules/dashboard/inputs/types";

export const useInputsSettings = createSettingsHook(
  "inputs",
  InputsSettingsSchema,
);
