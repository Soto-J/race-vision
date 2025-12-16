import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

interface PitHelperViewProps {
  title: string;
}
export const PitHelperView = ({ title }: PitHelperViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Indicator for pit speed and throttle control when you drive in pitlane."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
