import { InputsGraphView } from "@/modules/dashboard/inputs-graph/view";

import { createFileRoute } from "@tanstack/react-router";

const PAGE_TITLE = "inputs-graph" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: InputsGraph,
});

export default function InputsGraph() {
  return <InputsGraphView title={PAGE_TITLE} />;
}
