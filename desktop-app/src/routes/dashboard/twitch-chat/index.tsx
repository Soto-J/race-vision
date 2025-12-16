import { createFileRoute } from "@tanstack/react-router";

import { TwitchChatView } from "@/modules/dashboard/twitch-chat/view";

const PAGE_TITLE = "twitch-chat";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TwitchChat,
});

export default function TwitchChat() {
  return <TwitchChatView title={PAGE_TITLE} />;
}
