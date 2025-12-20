// import { z } from "zod";

// import { useMutation } from "@tanstack/react-query";

// import { settingsStore } from "@/lib/tauri-store";
// import { settingsKeys } from "@/lib/constants/query-keys";
// import { queryClient } from "@/routes/__root";

// export function useUpdateSettings<T>(page: string, schema: z.ZodSchema<T>) {
//   return useMutation({
//     mutationFn: async (next: T) => {
//       // Validate before saving
//       schema.parse(next);

//       await settingsStore.set(`pages.${page}`, next);

//       return next;
//     },

//     // Optimistic update
//     onMutate: async (next) => {
//       await queryClient.cancelQueries({
//         queryKey: settingsKeys.page(page),
//       });

//       const prev = queryClient.getQueryData<T>(settingsKeys.page(page));

//       queryClient.setQueryData(settingsKeys.page(page), next);

//       return { prev };
//     },

//     onError: (_err, _next, ctx) => {
//       if (ctx?.prev) {
//         queryClient.setQueryData(settingsKeys.page(page), ctx.prev);
//       }
//     },
//   });
// }
