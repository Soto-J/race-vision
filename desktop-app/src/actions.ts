import { invoke } from "@tauri-apps/api/core";
import { VarKindSchema } from "./lib/types";
import { TelemetryVars } from "./lib/constants/telemetry-vars";

export async function greet(name: string) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  return await invoke("greet", { name });
}

async function read_value() {
  try {
    const value = VarKindSchema.parse(
      await invoke("read_value", {
        key: TelemetryVars.SESSION_TIME,
      })
    );

    console.log("Value: ", value);

    if ("F64" in value) {
      return String(value.F64[0]);
    }
  } catch (error) {
    console.error("Failed to read session time:", error);
  }
}
