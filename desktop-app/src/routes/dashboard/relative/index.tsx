import { createFileRoute } from "@tanstack/react-router";

import { RelativeView } from "@/modules/dashboard/relative/view";

const PAGE_TITLE = "relative";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Relative,
});

export default function Relative() {
  return <RelativeView title={PAGE_TITLE} />;
}
