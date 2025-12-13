import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";

const PAGE_TITLE = "flat-map";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Flatmap,
});

export default function Flatmap() {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="Show drivers trackposition in a horizontal bar."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
