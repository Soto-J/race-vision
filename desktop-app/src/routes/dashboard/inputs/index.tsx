import { createFileRoute } from "@tanstack/react-router";

import { TelemetryVar, TelemetryVars } from "@/lib/constants/telemetry-vars";

import { InputsView } from "@/modules/dashboard/inputs/view";

const INPUT_VARS: TelemetryVar[] = [
  TelemetryVars.THROTTLE,
  TelemetryVars.BRAKE,
  TelemetryVars.CLUTCH,
];

const PAGE_TITLE = "inputs";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: Inputs,
});

export default function Inputs() {
  return <InputsView title={PAGE_TITLE} />;
}
