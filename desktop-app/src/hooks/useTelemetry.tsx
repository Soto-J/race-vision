import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";
import { VarKind } from "@/lib/types";
import { useTelemetryStore } from "./useTelemetryStore";

export type TelemetrySnapshot = Record<string, VarKind>;

export function useTelemetry() {
  useEffect(() => {
    // Bail out if not in Tauri or on overlay route
    if (!(window as any).__TAURI__) return;
    if (window.location.hash.startsWith("#/overlay/")) return;

    let unlisten: (() => void) | undefined;

    listen<TelemetrySnapshot>("telemetry-update", (event) => {
      useTelemetryStore.getState().setSnapshot(event.payload);
    }).then((fn) => (unlisten = fn));

    return () => unlisten?.();
  }, []);

  return null;
}
