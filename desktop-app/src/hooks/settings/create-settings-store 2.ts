import type { z } from "zod";

import { create } from "zustand";

import { immer } from "zustand/middleware/immer";

import { settingsStore } from "@/lib/tauri-store";

type TabKeys = "general" | "content" | "header" | "footer";

export function createSettingsStore<T extends z.ZodSchema>(
  pageKey: string,
  schema: T,
) {
  type Settings = z.infer<T>;

  interface StoreState {
    settings: Settings;
    isLoading: boolean;

    load: () => Promise<void>;
    togglePageActive: () => void;
    toggleFeature: (tab: TabKeys, feature: string) => void;
  }

  return create<StoreState>()(
    immer((set, get) => ({
      settings: schema.parse(undefined),
      isLoading: false,

      load: async () => {
        set({ isLoading: true });
        const settings = schema.parse(
          await settingsStore.get(`pages.${pageKey}`),
        );
        set({ settings, isLoading: false });
      },

      togglePageActive: () => {
        set((state) => {
          const next = {
            ...(state.settings as Record<string, any>),
            isActive: !(state.settings as any).isActive,
          } as Settings;
          settingsStore.set(`pages.${pageKey}`, next);
          return { settings: next };
        });
      },

      toggleFeature: (tab, feature) => {
        const settings = get().settings as any;

        const next = {
          ...settings,
          [tab]: {
            ...settings[tab],
            [feature]: {
              ...settings[tab][feature],
              isActive: !settings[tab][feature].isActive,
            },
          },
        } as Settings;

        settingsStore.set(`pages.${pageKey}`, next);
        set({ settings: next });
      },
    })),
  );
}
