import { useMemo } from "react";

import { createFileRoute } from "@tanstack/react-router";

import { InputsView } from "@/modules/dashboard/inputs/view";
import { usePageSettingsStore } from "@/hooks/store/use-page-store";
import { InputsSettingsSchema } from "@/modules/dashboard/inputs/types";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  const { setPageActive, settings } = usePageSettingsStore((s) => ({
    setPageActive: s.setPageActive,
    settings: s.pages[PAGE_TITLE],
  }));

  if (!settings) {
    return null;
  }

  const parsedSettings = useMemo(
    () => InputsSettingsSchema.parse(settings),
    [settings],
  );

  return (
    <InputsView
      page={PAGE_TITLE}
      settings={parsedSettings}
      setPageActive={setPageActive}
    />
  );
}
