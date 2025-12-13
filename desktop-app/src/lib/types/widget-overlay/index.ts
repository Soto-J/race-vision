import { z } from "zod";

/// Schemas
export const WidgetIdsSchema = z.enum([
  "inputs-widget",
  "standings-widget",
  "track-map-widget",
  "relative-widget",
]);

export const WidgetLayoutSchema = z.object({
  x: z.number(),
  y: z.number(),
  width: z.number(),
  height: z.number(),
  visible: z.boolean(),
});

export const WidgetsSchema = z.record(
  WidgetIdsSchema,
  WidgetLayoutSchema.optional(),
);

// Types
export type WidgetIds = z.infer<typeof WidgetIdsSchema>;
export type WidgetLayout = z.infer<typeof WidgetLayoutSchema>;
export type Widgets = z.infer<typeof WidgetsSchema>;
