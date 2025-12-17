import { z } from "zod";

import { useQuery } from "@tanstack/react-query";

import { load } from "@tauri-apps/plugin-store";

export const settingsKeys = {
  all: ["settings"] as const,
  page: (page: string) => [...settingsKeys.all, "page", page] as const,
};

export function usePageSettingsQuery<T>(page: string, schema: z.ZodSchema<T>) {
  return useQuery({
    queryKey: settingsKeys.page(page),
    queryFn: () => loadPageSettings(page, schema),
    staleTime: Infinity, // settings don’t go stale
    gcTime: Infinity,
  });
}

export async function loadPageSettings<T>(
  page: string,
  schema: z.ZodSchema<T>,
): Promise<T> {
  const store = await load("settings.json");

  const raw = await store.get(`pages.${page}`);
  const parsed = schema.safeParse(raw);

  if (!parsed.success) {
    console.error("Invalid settings", parsed.error);
    throw new Error(`Invalid settings for page: ${page}`);
  }

  return parsed.data;
}
