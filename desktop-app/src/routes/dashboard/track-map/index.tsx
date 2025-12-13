import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";

import { Switch } from "@/components/ui/switch";

const PAGE_TITLE = "track-map";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TrackMap,
});

export default function TrackMap() {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="See where everybody is on track."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div>Show overlay when</div>

        <div className="flex gap-x-8">
          <div className="flex items-center gap-x-2">
            <Switch />
            <span>In car</span>
          </div>
          <div className="flex items-center gap-x-2">
            <Switch />
            <span>In car</span>
          </div>
        </div>
      </Activity>
    </div>
  );
}
