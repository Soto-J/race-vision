import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";

export default function Flatmap() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["flat-map"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="flat-map"
        title="Flat Map"
        description="Show drivers trackposition in a horizontal bar."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
