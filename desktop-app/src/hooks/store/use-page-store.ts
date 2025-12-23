import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

export type PageSettingsMap = Record<string, unknown>;

type PageStoreSettingsState = {
  pages: Record<string, PageSettingsMap>;

  loadPages: () => Promise<void>;
  getPage: (page: string) => PageSettingsMap;
  setPageActive: (page: string, isActive: boolean) => Promise<void>;
  updateSettings: (page: string) => Promise<void>;
};

export const usePageSettingsStore = create<PageStoreSettingsState>()(
  immer((set, get) => ({
    pages: {},

    loadPages: async () => {
      const result = await invoke<Record<string, PageSettingsMap>>(
        "get_all_page_settings",
      );

      const normalized = Object.fromEntries(
        Object.entries(result).map(([page, settings]) => [
          page,
          normalizeKeys(settings),
        ]),
      );

      set((state) => {
        state.pages = normalized;
      });
    },

    getPage: (page: string) => {
      return get().pages[page];
    },

    setPageActive: async (page, isActive) => {
      await invoke("set_page_active", { page, is_active: isActive });

      set((state) => {
        state.pages[page].isActive = isActive;
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

const normalizeKeys = (obj: unknown): unknown => {
  if (typeof obj !== "object" || obj === null) {
    return obj;
  }

  if (Array.isArray(obj)) {
    return obj.map(normalizeKeys);
  }

  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    const camelKey = snakeToCamel(key);
    result[camelKey] = normalizeKeys(value);
  }

  return result;
};
