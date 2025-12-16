import { createFileRoute } from "@tanstack/react-router";

import { FlatMapView } from "@/modules/dashboard/flat-map/view";

const PAGE_TITLE = "flat-map" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Flatmap,
});

export default function Flatmap() {
  return <FlatMapView title={PAGE_TITLE} />;
}
