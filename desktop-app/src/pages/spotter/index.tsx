import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";
import { Switch } from "@/components/ui/switch";

export default function Spotter() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["inputs"] ?? false,
  );

  return (
    <div>
      <PageHeader
        id="spotter"
        title="Spotter"
        isActive={isActive}
        description="The spotter indicates when a driver is side by side."
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
