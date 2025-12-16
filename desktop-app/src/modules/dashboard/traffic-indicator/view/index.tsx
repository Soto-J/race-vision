import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

interface TrafficIndicatorViewProps {
  title: string;
}

export const TrafficIndicatorView = ({ title }: TrafficIndicatorViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? 0;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Give a warning/notification in multiclass sessions. When there is a faster class driver within 3 seconds behind you, then this window becomes active."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
