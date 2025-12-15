import { invoke } from "@tauri-apps/api/core";

const telemetryKey = "telemetry";
export const TELEMETRY_QUERY_KEYS = {
  all: [telemetryKey] as const,
  snapshot: [telemetryKey, "snapshot"] as const,
  inputs: [telemetryKey, "inputs"] as const,
};

////////////////////////////////////////
export async function greet(name: string) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("greet", { name });
}

// export const telemetryKeys = {
//   all: ["telemetry"] as const,
//   snapshot: () => [...telemetryKeys.all, "snapshot"] as const,
//   inputs: () => [...telemetryKeys.all, "inputs"] as const,
// };
