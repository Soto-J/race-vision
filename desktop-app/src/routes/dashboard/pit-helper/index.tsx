import { createFileRoute } from "@tanstack/react-router";

import { PitHelperView } from "@/modules/dashboard/pit-helper/view";

const PAGE_TITLE = "pit-helper" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: PitHelper,
});

export default function PitHelper() {
  return <PitHelperView title={PAGE_TITLE} />;
}
