import { createFileRoute } from "@tanstack/react-router";
import { DeltaBarView } from "@/modules/dashboard/delta-bar/view";

const PAGE_TITLE = "delta-bar";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: DeltaBar,
});

export default function DeltaBar() {
  return <DeltaBarView title={PAGE_TITLE} />;
}
