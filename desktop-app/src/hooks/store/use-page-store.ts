import { invoke } from "@tauri-apps/api/core";

import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { IndexMap, IndexMapSchema, PageConfig } from "./types";

type PageStoreSettingsState = {
  index: IndexMap;

  loadPagesSettings: () => Promise<void>;
  getPage: (page: string) => PageConfig | undefined;
  setPageActive: (page: string, isActive: boolean) => Promise<void>;
  updateSettings: (page: string) => Promise<void>;
};

export const usePageSettingsStore = create<PageStoreSettingsState>()(
  immer((set, get) => ({
    index: {},

    loadPagesSettings: async () => {
      const result = await invoke("get_all_page_settings");

      const settings = IndexMapSchema.parse(result);

      set((state) => {
        state.index = settings;
      });
    },

    getPage: (page: string) => {
      return get().index[page];
    },

    setPageActive: async (page, isActive) => {
      await invoke("set_page_active", { page, isActive });

      set((state) => {
        if (!state.index[page]) {
          return;
        }

        state.index[page].isActive = isActive;
      });
    },

    updateSettings: async (page) => {
      const pageConfig = get().index[page];

      const result = await invoke("update_setting", {
        page,
        settings: pageConfig,
      });
    },
  })),
);
