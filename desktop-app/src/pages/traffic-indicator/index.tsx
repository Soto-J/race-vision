import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";

export default function TrafficIndicator() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["traffic-indicator"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="traffic-indicator"
        title="Traffic Indicator"
        description="Give a warning/notification in multiclass sessions. When there is a faster class driver within 3 seconds behind you, then this window becomes active."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
