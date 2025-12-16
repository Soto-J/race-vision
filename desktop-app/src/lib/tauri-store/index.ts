import { LazyStore, Store, load } from "@tauri-apps/plugin-store";
import { z } from "zod";

const TELEMETRY_VARS = "telemetry-vars.json";
const PAGE_SETTINGS = "page-settings.json";

export const telemetryVarStore = new LazyStore(TELEMETRY_VARS);

export const pageSettingsStore = new LazyStore(PAGE_SETTINGS);
