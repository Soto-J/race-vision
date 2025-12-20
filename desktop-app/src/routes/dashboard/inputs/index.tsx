import { createFileRoute } from "@tanstack/react-router";

import { InputsSettingsSchema } from "@/modules/dashboard/inputs/types";
import { InputsView } from "@/modules/dashboard/inputs/view";
// import { usePageSettings } from "@/hooks/settings/use-page-settings";

const PAGE_TITLE = "inputs" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  // const {
  //   data: settings,
  //   isLoading,
  //   error,
  // } = usePageSettings(PAGE_TITLE, InputsSettingsSchema, {});

  // if (isLoading) {
  //   return null;
  // }

  // if (error || !settings) {
  //   return <div>Error</div>;
  // }

  return (
    <InputsView
      title={PAGE_TITLE}
      settings={1}
      schema={InputsSettingsSchema}
    />
  );
}
