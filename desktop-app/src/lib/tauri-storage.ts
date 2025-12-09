import { getStore } from "@tauri-apps/plugin-store";

let storePromise: Promise<any> | null = null;

async function getWidgetStore() {
  if (!storePromise) {
    storePromise = getStore("widget-layouts.json");
  }
  return storePromise;
}

const getItem = async (key: string) => {
  const store = await getWidgetStore();
  return await store.get(key);
};

const setItem = async (key: string, value: any) => {
  const store = await getWidgetStore();
  await store.set(key, value);
  await store.save();
};

const removeItem = async (key: string) => {
  const store = await getWidgetStore();
  await store.delete(key);
  await store.save();
};

export const TauriStorage = {
  getItem,
  setItem,
  removeItem,
};
