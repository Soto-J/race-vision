import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";

interface TwitchChatViewProps {
  title: string;
}
export const TwitchChatView = ({ title }: TwitchChatViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
        description="Chat overlay for Twitch streaming."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
};
