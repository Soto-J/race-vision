import { telemetryVarStore } from ".";

export async function loadActiveVars(page: string): Promise<Set<string>> {
  const vars = (await telemetryVarStore.get<string[]>(`${page}.vars`)) ?? [];

  return new Set(vars);
}

export async function saveActiveVars(page: string, vars: Set<string>) {
  await telemetryVarStore.set(`${page}.vars`, Array.from(vars));
  
  await telemetryVarStore.save();
}
