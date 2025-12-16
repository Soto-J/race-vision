import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

interface DeltaBarViewProps {
  title: string;
}

export const DeltaBarView = ({ title }: DeltaBarViewProps) => {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Show your delta times in a horizontal bar and more."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
