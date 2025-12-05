import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

import { parseToValue } from "@/lib/types";
import { TelemetrySnapshot } from "./useTelemetry";
import { invoke } from "@tauri-apps/api/core";

type TelemetryStore = {
  // State
  data: Record<string, number | boolean | string>;
  isActive: Record<string, boolean>;
  pageVars: Record<string, string[]>;

  // Action
  toggleSetting: (id: string, value: boolean, vars?: string[]) => void;
  setSnapshot: (snap: TelemetrySnapshot) => void;
  setPageVars: (id: string, vars: string[]) => void;
  syncToRust: () => void;
  getValue: (varName: string) => number | boolean | string | undefined;
};

export const useTelemetryStore = create<TelemetryStore>()(
  immer((set, get) => ({
    data: {},
    isActive: {},
    pageVars: {},

    toggleSetting: (id, value, vars = []) => {
      set((state) => {
        state.isActive[id] = value;

        // If enabling → store required vars
        // If disabling → clear vars for this page
        state.pageVars[id] = value ? vars : [];
      });

      get().syncToRust();
    },

    setSnapshot: (snapshot) => {
      set((state) => {
        for (const [key, varKind] of Object.entries(snapshot)) {
          state.data[key] = parseToValue(varKind);
        }
      });
    },

    setPageVars: (id, vars) => {
      set((state) => (state.pageVars[id] = vars));

      get().syncToRust();
    },

    syncToRust: () => {
      const all = Object.values(get().pageVars).flat();
      invoke("set_watched_vars", { vars: all });
    },

    getValue: (key) => get().data[key],
  }))
);
