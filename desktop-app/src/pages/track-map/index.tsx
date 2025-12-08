import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";

import { Switch } from "@/components/ui/switch";

export default function TrackMap() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["track-map"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="track-map"
        title="Track Map"
        description="See where everybody is on track."
        vars={[]}
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
