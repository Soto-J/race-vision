export const settingsKeys = {
  all: ["settings"] as const,
  page: (page: string) => [...settingsKeys.all, "page", page] as const,
};
