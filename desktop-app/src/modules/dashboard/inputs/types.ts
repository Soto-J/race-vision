import { z } from "zod";

const GeneralSchema = z.object({});
const ContentSchema = z.object({
  testing: z.boolean().default(false),
});

const HeaderSchema = z.object({});

export const InputsSettingsSchema = z.object({
  content: ContentSchema,
});
