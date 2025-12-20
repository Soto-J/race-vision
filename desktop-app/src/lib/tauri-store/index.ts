import { LazyStore, load } from "@tauri-apps/plugin-store";

export const TELEMETRY_VARS_JSON = "telemetry-vars.json" as const;
export const APP_SETTINGS_JSON = "app-settings.json" as const;

interface UserSettings {
  theme: "dark" | "light";
  notifications: boolean;
  volume: number;
}

const test = await load(APP_SETTINGS_JSON, {
  defaults: {},
  autoSave: true,
});

export const telemetryVarStore = new LazyStore(TELEMETRY_VARS_JSON);

export const settingsStore = new LazyStore(APP_SETTINGS_JSON, {
  defaults: {},
  autoSave: true,
});
