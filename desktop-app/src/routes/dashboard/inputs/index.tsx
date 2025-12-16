import { createFileRoute } from "@tanstack/react-router";

import { usePageSettingsQuery } from "@/hooks/store/use-settings-query";

import { InputsSettingsSchema } from "@/modules/dashboard/inputs/types";
import { InputsView } from "@/modules/dashboard/inputs/view";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  const {
    data: settings,
    isLoading,
    error,
  } = usePageSettingsQuery(PAGE_TITLE, InputsSettingsSchema);

  if (isLoading) {
    return null;
  }

  if (error || !settings) {
    return <div>Error</div>;
  }

  return (
    <InputsView
      title={PAGE_TITLE}
      settings={settings}
      schema={InputsSettingsSchema}
    />
  );
}
