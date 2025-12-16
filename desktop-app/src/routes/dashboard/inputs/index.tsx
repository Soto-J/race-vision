import { createFileRoute } from "@tanstack/react-router";

import { InputsView } from "@/modules/dashboard/inputs/view";

const PAGE_TITLE = "inputs";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  return <InputsView title={PAGE_TITLE} />;
}
