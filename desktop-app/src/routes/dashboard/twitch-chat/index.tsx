import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";

const PAGE_TITLE = "twitch-chat";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TwitchChat,
});

export default function TwitchChat() {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;
  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="Chat overlay for Twitch streaming."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <div></div>
      </Activity>
    </div>
  );
}
