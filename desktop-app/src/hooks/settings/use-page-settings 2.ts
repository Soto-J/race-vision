import { useQuery } from "@tanstack/react-query";
import { z } from "zod";
import { settingsStore } from "@/lib/tauri-store";
import { AppSettings, PAGE_SETTINGS } from "@/modules/dashboard/types";

type PageKey = keyof typeof PAGE_SETTINGS;
type PageSettings = (typeof PAGE_SETTINGS)[PageKey];

export const settingsKeys = {
  all: ["settings"] as const,
  page: (page: PageKey) => [...settingsKeys.all, page] as const,
};

export function usePageSettings<PageKey extends keyof typeof PAGE_SETTINGS>(
  page: PageKey,
) {
  const { schema, defaults } = PAGE_SETTINGS[page];

  return useQuery<PageSettings<PageKey>>({
    queryKey: settingsKeys.page(page),
    queryFn: async () => {
      let raw = await settingsStore.get(page);

      return schema.parse(raw ?? defaults);
    },
    initialData: defaults,
    staleTime: Infinity,
    gcTime: Infinity,
  });
}
