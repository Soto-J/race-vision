import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/pages/components/page-header";

export default function DeltaBar() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["delta-bar"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="delta"
        title="Delta Bar"
        description="Show your delta times in a horizontal bar and more."
        vars={[]}
      />
      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
