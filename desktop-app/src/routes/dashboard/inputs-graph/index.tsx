import { createFileRoute } from "@tanstack/react-router"
import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";


const PAGE_TITLE = "inputs";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});
export default function InputsGraph() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["inputs-graph"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="inputs-graph"
        title="Inputs Graph"
        description="Show the inputs graph in a separate window, so you can place this where you want on your screen."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
