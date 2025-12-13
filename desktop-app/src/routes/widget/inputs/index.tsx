import { createFileRoute } from "@tanstack/react-router";

import { PedalGraph } from "@/routes/widget/components/widget/pedal-graph";

export const Route = createFileRoute(`/widget/inputs/`)({
  component: InputsWidget,
});

export default function InputsWidget() {
  return (
    <div>
      <PedalGraph />
    </div>
  );
}
