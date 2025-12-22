import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

type PageSettingRow = {
  page: string;
  setting: string;
  value: number; // 0 | 1
};

export type PageSettings = {
  is_active?: boolean;
  // future:
  // display_race?: boolean;
  // display_practice?: boolean;
};

type PageStoreSettingsState = {
  pages: Record<string, PageSettings>;

  loadPage: (page: string) => Promise<PageSettings>;
  togglePage: (page: string) => Promise<boolean>;
  updateSettings: (page: string) => Promise<void>;
};

export const usePageSettingsStore = create<PageStoreSettingsState>()(
  immer((set, get) => ({
    pages: {},

    loadPage: async (page) => {
      const rows = await invoke<PageSettingRow[]>("page_settings", { page });

      const normalized: PageSettings = {};

      for (const { setting, value } of rows) {
        normalized[setting as keyof PageSettings] = value === 1;
      }

      set((state) => (state.pages[page] = normalized));

      return normalized;
    },

    togglePage: async (page) => {
      const isActive = await invoke<boolean>("toggle_page", { page });

      set((state) => {
        state.pages[page] ??= {};
        state.pages[page].is_active = isActive;
      });

      return isActive;
    },

    updateSettings: async (page) => {
      await invoke("update_setting")
    }
  })),
);
