import { createFileRoute } from "@tanstack/react-router";

import { TrackMapView } from "@/modules/dashboard/track-map/view";

const PAGE_TITLE = "track-map";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: TrackMap,
});

export default function TrackMap() {
  return <TrackMapView title={PAGE_TITLE} />;
}
