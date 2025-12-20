import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

import { invoke } from "@tauri-apps/api/core";

import {
  type TelemetryValue,
  type TelemetrySnapshot,
  TelemetryValueSchema,
} from "@/lib/types";
import type { TelemetryVar } from "@/lib/constants/telemetry-vars";

type TelemetryStore = {
  data: Record<string, TelemetryValue>;
  // Keeps track of which page is active
  pageIsActive: Record<string, boolean>;
  // Keeps track of which variables are active for a giving page
  pageVars: Record<string, TelemetryVar[]>;
  subscriptions: Set<TelemetryVar>;

  setSnapshot: (snap: TelemetrySnapshot) => void;

  togglePage: (id: string, value: boolean, vars?: TelemetryVar[]) => void;
  toggleVar: (pageId: string, varName: TelemetryVar, enabled: boolean) => void;

  setPageVars: (id: string, vars: TelemetryVar[]) => void;
  getValue: (varName: string) => TelemetryValue | undefined;
  syncToRust: () => void;
};

export const useTelemetryStore = create<TelemetryStore>()(
  immer((set, get) => ({
    data: {},
    pageIsActive: {},
    pageVars: {},
    subscriptions: new Set(),

    setSnapshot: (snapshot) => {
      set((state) => {
        for (const [key, varKind] of Object.entries(snapshot)) {
          const result = TelemetryValueSchema.safeParse(varKind);

          if (!result.success) {
            console.warn("Problem parsing telemetry value", {
              key,
              error: result.error,
              raw: varKind,
            });

            continue;
          }

          state.data[key] = result.data;
        }
      });
    },

    togglePage: (pageId, isEnabled) => {
      set((state) => {
        state.pageIsActive[pageId] = isEnabled;
        recomputeSubscriptions(state);
      });

      get().syncToRust();
    },

    setPageVars: (id, vars) => {
      set((state) => {
        state.pageVars[id] = vars;
        recomputeSubscriptions(state);
      });

      get().syncToRust();
    },

    toggleVar: (pageId, varName, enabled) => {
      set((state) => {
        const vars = state.pageVars[pageId] ?? [];

        if (enabled) {
          if (!vars.includes(varName)) {
            vars.push(varName);
          }

          state.pageVars[pageId] = vars;
        } else {
          state.pageVars[pageId] = vars.filter((v) => v !== varName);
        }

        recomputeSubscriptions(state);
      });
    },

    syncToRust: () => {
      // sends a list of watched variables to Rust.
      invoke("set_watched_vars", {
        vars: Array.from(get().subscriptions),
      }).catch((error) =>
        console.log(`Failed to sync watched variables to Rust: ${error}`),
      );
    },

    getValue: (key) => get().data[key],
  })),
);

const recomputeSubscriptions = (state: TelemetryStore) => {
  const activeVars = new Set<TelemetryVar>();

  Object.entries(state.pageVars).forEach(([pageId, list]) => {
    if (!state.pageVars[pageId]) return;

    list.forEach((v) => activeVars.add(v));
  });

  state.subscriptions = activeVars;
};
