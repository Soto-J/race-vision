import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

import { parseToValue } from "@/lib/types";
import { TelemetrySnapshot } from "./useTelemetry";

type TelemetryData = Record<string, number | boolean | string>;

type State = {
  data: TelemetryData;
};

type Actions = {
  setSnapshot: (snap: TelemetrySnapshot) => void;
  get: (varName: string) => number | boolean | string | undefined;
};

export const useTelemetryStore = create<State & Actions>()(
  immer((set, get) => ({
    data: {},

    setSnapshot: (snapshot) => {
      set((state) => {
        for (const key in snapshot) {
          state.data = parseToValue(snapshot[key]);
        }
      });
    },
    get: (key) => get().data[key],
  }))
);
