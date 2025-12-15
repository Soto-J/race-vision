import { createFileRoute } from "@tanstack/react-router";
import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

const PAGE_TITLE = "delta-bar";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: DeltaBar,
});

export default function DeltaBar() {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="Show your delta times in a horizontal bar and more."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
