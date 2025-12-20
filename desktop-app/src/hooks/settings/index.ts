// import { settingsStore } from "@/lib/tauri-store";
// import { SCHEMA_MAP } from "@/modules/dashboard/types";
// import { useQuery } from "@tanstack/react-query";
// import { z } from "zod";

// export function createSettingsHook<T extends z.ZodSchema>(
//   key: string,
//   schema: T,
// ) {
//   return function useSettings() {
//     return useQuery({
//       queryKey: ["settings", key],
//       queryFn: async () => schema.parse(await settingsStore.get(key)),
//       // initialData: schema.parse(undefined),
//       staleTime: Infinity,
//       gcTime: Infinity,
//     });
//   };
// }

// export function useGetSettings(key: keyof typeof SCHEMA_MAP) {
//   const schema = SCHEMA_MAP[key];

//   return useQuery({
//     queryKey: ["settings", key],
//     queryFn: async () => schema.parse(await settingsStore.get(key)),
//     staleTime: Infinity,
//     gcTime: Infinity,
//   });
// }
