
import { createFileRoute } from "@tanstack/react-router";


import { SpotterView } from "@/modules/dashboard/spotter/view";

const PAGE_TITLE = "spotter";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Spotter,
});

export default function Spotter() {
  return <SpotterView title={PAGE_TITLE} />;
}
