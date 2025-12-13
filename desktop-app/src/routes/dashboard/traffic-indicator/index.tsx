import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";

const PAGE_TITLE = "traffic-indicator";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TrafficIndicator,
});

export default function TrafficIndicator() {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="Give a warning/notification in multiclass sessions. When there is a faster class driver within 3 seconds behind you, then this window becomes active."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
