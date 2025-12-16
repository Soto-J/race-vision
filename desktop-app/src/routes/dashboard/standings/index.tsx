import { createFileRoute } from "@tanstack/react-router";

import { StandingsView } from "@/modules/dashboard/standings/view";

const PAGE_TITLE = "standings";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Standings,
});

export default function Standings() {
  return <StandingsView title={PAGE_TITLE} />;
}
