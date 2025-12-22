import { useEffect } from "react";

import { createFileRoute } from "@tanstack/react-router";

import { InputsView } from "@/modules/dashboard/inputs/view";
import { usePageSettingsStore } from "@/hooks/store/use-page-store";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  const loadPage = usePageSettingsStore((s) => s.loadPage);
  const settings = usePageSettingsStore((s) => s.pages[PAGE_TITLE]);

  useEffect(() => {
    loadPage(PAGE_TITLE);
  }, [loadPage, settings]);

  if (!settings) {
    return null;
  }

  return <InputsView page={PAGE_TITLE} settings={settings} />;
}
