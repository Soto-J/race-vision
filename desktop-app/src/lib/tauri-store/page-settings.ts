import { pageSettingsStore } from ".";

export async function loadPageSettings(page: string): Promise<Set<string>> {
  const settings =
    (await pageSettingsStore.get<string[]>(`${page}.setting`)) ?? [];

  return new Set(settings);
}

export async function savePageSettings(page: string, vars: Set<string>) {
  await pageSettingsStore.set(`${page}.setting`, Array.from(vars));
  await pageSettingsStore.save();
}
