import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

import type { TelemetrySnapshot } from "@/lib/types";

import { useQueryClient } from "@tanstack/react-query";
import { TELEMETRY_QUERY_KEYS } from "@/lib/constants/telemetry-keys";

const TELEMETRY_UPDATE_EVENT = "telemetry-update";

export function useTelemetryListener() {
  const queryClient = useQueryClient();

  useEffect(() => {
    // Bail out if not in Tauri or on overlay route
    if (!(window as any).__TAURI__) return;
    if (window.location.hash.startsWith("#/")) return;

    let unlisten: (() => void) | undefined;

    listen<TelemetrySnapshot>(TELEMETRY_UPDATE_EVENT, (event) => {
      queryClient.setQueryData(TELEMETRY_QUERY_KEYS.snapshot, event.payload);
    }).then((fn) => (unlisten = fn));

    return () => unlisten?.();
  }, [queryClient]);

  return null;
}
