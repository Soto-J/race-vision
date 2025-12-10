import { z } from "zod";

/// Schemas
export const WingDataSchema = z.object({
  x: z.number(),
  y: z.number(),
  width: z.number(),
  height: z.number(),
});

export const WidgetsSchema = z.record(z.string(), WingDataSchema);

// Types
export type WidgetData = z.infer<typeof WingDataSchema>;
export type Widgets = z.infer<typeof WidgetsSchema>;


