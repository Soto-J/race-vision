import { Activity } from "react";
import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

interface FlatMapViewProps {
  title: string;
}

export const FlatMapView = ({ title }: FlatMapViewProps) => {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;

  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Show drivers trackposition in a horizontal bar."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
