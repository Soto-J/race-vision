import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/pages/components/page-header";

export default function PitHelper() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["pit-helper"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="pit-helper"
        title="Pit Helper"
        description="Indicator for pit speed and throttle control when you drive in pitlane."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
