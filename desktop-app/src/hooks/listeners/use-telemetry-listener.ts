import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

import { VarKind } from "@/lib/types";

import { useTelemetryStore } from "../store/use-telemetry-store";

export type TelemetrySnapshot = Record<string, VarKind>;

export function useTelemetryListener() {
  const { setSnapshot, syncToRust, subscriptions } = useTelemetryStore();

  useEffect(() => {
    // Bail out if not in Tauri or on overlay route
    if (!(window as any).__TAURI__) return;
    if (window.location.hash.startsWith("#/")) return;

    let unlisten: (() => void) | undefined;

    listen<TelemetrySnapshot>("telemetry-update", (event) => {
      setSnapshot(event.payload);
    }).then((fn) => (unlisten = fn));

    return () => unlisten?.();
  }, []);

  // Subscription → Rust sync
  useEffect(() => {
    if (!(window as any).__TAURI__) return;

    syncToRust();
  }, [
    syncToRust,
    // force stable comparison
    Array.from(subscriptions).sort().join(","),
  ]);

  return null;
}
