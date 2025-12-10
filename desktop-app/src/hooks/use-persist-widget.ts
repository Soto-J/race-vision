import { useEffect } from "react";

import { getStore } from "@tauri-apps/plugin-store";

import { WidgetsSchema } from "@/lib/types/widget-overlay";

import { useOverlayStore } from "@/hooks/store/use-overlay-store";

let saveTimer: ReturnType<typeof setTimeout> | null = null;

export function usePersistWidget() {
  const widgets = useOverlayStore((s) => s.widgets);

  useEffect(() => {
    if (saveTimer) clearTimeout(saveTimer);

    saveTimer = setTimeout(async () => {
      const store = await getStore("widget-layouts.json");

      if (!store) {
        console.error("Failed to load widget-layouts.json store");
        return;
      }

      // Validate widgets before saving
      const result = WidgetsSchema.safeParse(widgets);

      if (!result.success) {
        console.warn("Skipping save — invalid widget state", result.error);
        return;
      }

      await store.set("widgets", result.data);
      await store.save();
    }, 200);
  }, [widgets]);
}
