import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";
import { PageHeader } from "@/modules/dashboard/components/page-header";

interface InputsGraphViewProps {
  title: string;
}
export const InputsGraphView = ({ title }: InputsGraphViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? 0;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Show the inputs graph in a separate window, so you can place this where you want on your screen."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
