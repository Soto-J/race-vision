import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/pages/components/page-header";

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
