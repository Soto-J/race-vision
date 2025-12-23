import { useEffect, useMemo } from "react";

import { createFileRoute } from "@tanstack/react-router";

import { InputsView } from "@/modules/dashboard/inputs/view";
import { usePageSettingsStore } from "@/hooks/store/use-page-store";
import { InputsSettingsSchema } from "@/modules/dashboard/inputs/types";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  const { loadPage, setPageActive, settings } = usePageSettingsStore((s) => ({
    loadPage: s.loadPage,
    setPageActive: s.setPageActive,
    settings: s.page[PAGE_TITLE],
  }));

  useEffect(() => {
    loadPage(PAGE_TITLE);
  }, [loadPage]);

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
