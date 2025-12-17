import { z } from "zod";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { settingsKeys } from "./use-settings-query";

import { LazyStore } from "@tauri-apps/plugin-store";

const settingsStore = new LazyStore("settings.json");

export function useUpdateSettings<T>(page: string, schema: z.ZodSchema<T>) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (next: T) => {
      // Validate before saving
      schema.parse(next);

      await settingsStore.set(`pages.${page}`, next);
      await settingsStore.save();

      return next;
    },

    // Optimistic update
    onMutate: async (next) => {
      await queryClient.cancelQueries({
        queryKey: settingsKeys.page(page),
      });

      const prev = queryClient.getQueryData<T>(settingsKeys.page(page));

      queryClient.setQueryData(settingsKeys.page(page), next);

      return { prev };
    },

    onError: (_err, _next, ctx) => {
      if (ctx?.prev) {
        queryClient.setQueryData(settingsKeys.page(page), ctx.prev);
      }
    },

    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: settingsKeys.page(page),
      });
    },
  });
}
