import { createFileRoute } from "@tanstack/react-router";

import { TrafficIndicatorView } from "@/modules/dashboard/traffic-indicator/view";

const PAGE_TITLE = "traffic-indicator";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TrafficIndicator,
});

export default function TrafficIndicator() {
  return <TrafficIndicatorView title={PAGE_TITLE} />;
}
