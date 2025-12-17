import { LazyStore } from "@tauri-apps/plugin-store";

const TELEMETRY_VARS = "telemetry-vars.json";
const PAGE_SETTINGS = "settings.json";

export const telemetryVarStore = new LazyStore(TELEMETRY_VARS);

export const settingsStore = new LazyStore(PAGE_SETTINGS);
