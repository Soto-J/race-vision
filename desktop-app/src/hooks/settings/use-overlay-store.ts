import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

import { WidgetData, Widgets } from "@/lib/types/widget-overlay";

type OverlayStore = {
  widgets: Widgets;
  isInEditMode: boolean;

  toggleEditMode: (mode: boolean) => void;
  registerWidget: (id: string, data: WidgetData) => void;
  updateWidget: (id: string, data: Partial<WidgetData>) => void;
};

export const useOverlayStore = create<OverlayStore>()(
  immer((set) => ({
    widgets: {},
    isInEditMode: false,

    toggleEditMode: (mode) =>
      set((state) => {
        state.isInEditMode = mode;
      }),

    registerWidget: (id, data) =>
      set((state) => {
        state.widgets[id] = data;
      }),

    updateWidget: (id, data) =>
      set((state) => {
        if (!state.widgets[id]) return;

        Object.assign(state.widgets[id], data);
      }),
  })),
);
