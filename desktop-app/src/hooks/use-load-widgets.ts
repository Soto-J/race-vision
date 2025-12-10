import { useEffect } from "react";
import { z } from "zod";

import { getStore } from "@tauri-apps/plugin-store";

import { useOverlayStore } from "@/hooks/store/use-overlay-store";

import { WidgetsSchema } from "@/lib/types/widget-overlay";

export function useLoadWidgets() {
  useEffect(() => {
    const load = async () => {
      const store = await getStore("widget-layouts.json");
      const raw = await store?.get("widgets");

      const result = WidgetsSchema.safeParse(raw);

      if (!result.success) {
        console.error(`widgets parse failed: ${z.treeifyError(result.error)}`);
        return;
      }

      useOverlayStore.setState({ widgets: result.data });
    };

    load();
  }, []);
}
