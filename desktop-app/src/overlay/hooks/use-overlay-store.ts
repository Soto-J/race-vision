import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

type WidgetData = {
  x: number;
  y: number;
  width: number;
  height: number;
};

type Widgets = Record<string, WidgetData>;

type OverlayStore = {
  widgets: Widgets;
  isInEditMode: boolean;

  toggleEditMode: () => void;
  registerWidget: (id: string, data: WidgetData) => void;
  updateWidget: (id: string, data: Partial<WidgetData>) => void;
};

export const useOverlayStore = create<OverlayStore>()(
  immer((set) => ({
    widgets: {},
    isInEditMode: false,

    toggleEditMode: () =>
      set((state) => {
        state.isInEditMode = !state.isInEditMode;
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
