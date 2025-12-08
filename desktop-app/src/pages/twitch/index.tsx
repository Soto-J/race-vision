import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";

export default function TwitchChat() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["pit-helper"] ?? false,
  );
  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="twitch-chat"
        title="Twitch Chat"
        description="Chat overlay for Twitch streaming."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
