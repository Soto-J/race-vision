import { Activity } from "react";
import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

import { Switch } from "@/modules/components/ui/switch";

interface SpotterViewProps {
  title: string;
}

export const SpotterView = ({ title }: SpotterViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
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
};
