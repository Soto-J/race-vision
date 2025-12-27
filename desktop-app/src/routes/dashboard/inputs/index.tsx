import { createFileRoute } from "@tanstack/react-router";

import { InputsView } from "@/modules/dashboard/inputs/view";
import { usePageSettingsStore } from "@/hooks/store/use-page-store";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  const setPageActive = usePageSettingsStore((s) => s.setPageActive);
  const settings = usePageSettingsStore((s) => s.index[PAGE_TITLE]);
  const updateSetting = usePageSettingsStore().updateSettings;

  if (!settings) {
    return null;
  }

  return (
    <InputsView
      page={PAGE_TITLE}
      settings={settings}
      setPageActive={setPageActive}
      updateSettings={updateSetting}
    />
  );
}
