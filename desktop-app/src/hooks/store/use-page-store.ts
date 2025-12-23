import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

export type PageSettingsMap = Record<string, boolean>;

type PageStoreSettingsState = {
  page: PageSettingsMap;
  allPages: Record<string, PageSettingsMap>;

  loadPage: (page: string) => Promise<PageSettingsMap>;
  loadAllPages: () => Promise<Record<string, PageSettingsMap>>;
  setPageActive: (page: string, isActive: boolean) => Promise<void>;
  updateSettings: (page: string) => Promise<void>;
};

export const usePageSettingsStore = create<PageStoreSettingsState>()(
  immer((set, get) => ({
    page: {},
    allPages: {},

    loadPage: async (page) => {
      const result = await invoke<Record<string, boolean>>(
        "get_page_settings",
        { page },
      );

      const normalized = normalizeSettings(result);

      set((state) => {
        state.page = normalized;
      });

      return normalized;
    },

    loadAllPages: async () => {
      const result = await invoke<Record<string, PageSettingsMap>>(
        "get_all_page_settings",
      );

      const normalized = Object.fromEntries(
        Object.entries(result).map(([page, settings]) => [
          page,
          normalizeSettings(settings),
        ]),
      );

      set((state) => {
        state.allPages = normalized;
      });

      return normalized;
    },

    setPageActive: async (page, isActive) => {
      await invoke("set_page_active", { page, is_active: isActive });

      set((state) => {
        state.page ??= {};
        state.page.isActive = isActive;
      });
    },

    updateSettings: async (page) => {
      //TODO next
      await invoke("update_setting");
    },
  })),
);

const snakeToCamel = (s: string) =>
  s.replace(/_([a-z])/g, (_, c) => c.toUpperCase());

const normalizeSettings = (settings: Record<string, boolean>) =>
  Object.fromEntries(
    Object.entries(settings).map(([k, v]) => [snakeToCamel(k), v]),
  );
