import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

import { Switch } from "@/modules/components/ui/switch";

interface TrackMapViewProps {
  title: string;
}

export const TrackMapView = ({ title }: TrackMapViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? 0;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
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
};
