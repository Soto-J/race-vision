import { createFileRoute } from "@tanstack/react-router";

import { FuelCalculatorView } from "@/modules/dashboard/fuel-calculator/view";

const PAGE_TITLE = "fuel-calculator" as const;

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: FuelCalculator,
});

export default function FuelCalculator() {
  return <FuelCalculatorView title={PAGE_TITLE} />;
}
