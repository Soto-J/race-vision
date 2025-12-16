import { settingsStore } from ".";

export async function savePageSettings(page: string, vars: Set<string>) {
  await settingsStore.set(`${page}.setting`, Array.from(vars));
  await settingsStore.save();
}
