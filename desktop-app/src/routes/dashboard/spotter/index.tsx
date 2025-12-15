import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

import { Switch } from "@/modules/components/ui/switch";

const PAGE_TITLE = "spotter";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Spotter,
});

export default function Spotter() {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id="spotter"
        title="Spotter"
        description="The spotter indicates when a driver is side by side."
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
