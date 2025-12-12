import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

import { VarKind } from "@/lib/types";

import { useTelemetryStore } from "../store/use-telemetry-store";

export type TelemetrySnapshot = Record<string, VarKind>;

export function useTelemetryListener() {
  useEffect(() => {
    // Bail out if not in Tauri or on overlay route
    if (!(window as any).__TAURI__) return;

    if (window.location.hash.startsWith("#/")) return;

    // console.log("Telem listener mount check", window.location.hash);
    // console.log("Telemetry listener ACTIVATED");

    let unlisten: (() => void) | undefined;

    listen<TelemetrySnapshot>("telemetry-update", (event) => {
      useTelemetryStore.getState().setSnapshot(event.payload);
    }).then((fn) => (unlisten = fn));

    return () => unlisten?.();
  }, []);

  return null;
}
