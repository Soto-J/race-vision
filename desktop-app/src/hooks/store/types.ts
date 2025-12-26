import { z } from "zod";

// SCHEMAS
export const SessionSchema = z.enum(["practice", "qualy", "race"]);

export const DisplayInSchema = z.object({
  practice: z.boolean(),
  qualy: z.boolean(),
  race: z.boolean(),
});

export const SettingSchema = z.object({
  isActive: z.boolean(),
  displayIn: DisplayInSchema,
});

export const SectionSchema = z.record(z.string(), SettingSchema);

export const PageConfigSchema = z
  .object({
    isActive: z.boolean(),
  })
  .catchall(SectionSchema);

export const IndexMapSchema = z.record(z.string(), PageConfigSchema);

// TYPES
export type Session = z.infer<typeof SectionSchema>;
export type DisplaIn = z.infer<typeof DisplayInSchema>;
export type Setting = z.infer<typeof SettingSchema>;
export type Section = z.infer<typeof SectionSchema>;
export type PageConfig = z.infer<typeof PageConfigSchema>;
export type IndexMap = z.infer<typeof IndexMapSchema>;
