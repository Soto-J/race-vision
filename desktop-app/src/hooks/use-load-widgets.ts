import { useEffect } from "react";
import { z } from "zod";

import { getStore } from "@tauri-apps/plugin-store";

import { useOverlayStore } from "@/hooks/store/use-overlay-store";

import {
  type WidgetIds,
  type WidgetLayout,
  WidgetLayoutSchema,
} from "@/lib/types/widget-overlay";

export const WIDGET_IDS = [
  "pedals-widget",
  "standings-widget",
  "track-map-widget",
  "relative-widget",
] as const;

export function useLoadWidgets() {
  useEffect(() => {
    const load = async () => {
      const store = await getStore("widget-layouts.json");

      const widgets: Partial<Record<WidgetIds, WidgetLayout>> = {};

      for (const id of WIDGET_IDS) {
        const raw = await store?.get(id);
        // Missing widget is normal on first run
        if (raw === undefined) continue;

        const result = WidgetLayoutSchema.safeParse(raw);

        if (!result.success) {
          console.error(
            `widgets parse failed: ${z.treeifyError(result.error)}`,
          );
          continue;
        }

        widgets[id] = result.data;
      }

      useOverlayStore.setState((state) => ({
        widgets: {
          ...state.widgets,
          ...widgets,
        },
      }));
    };

    load().catch(console.error);
  }, []);
}
