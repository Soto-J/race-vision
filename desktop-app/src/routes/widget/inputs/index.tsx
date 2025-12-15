import { createFileRoute } from "@tanstack/react-router";

import { PedalGraph } from "@/modules/widget/components/pedal-graph";

export const Route = createFileRoute(`/widget/inputs/`)({
  component: InputsWidget,
});

export default function InputsWidget() {
  return (
    <div data-tauri-drag-region>
      <PedalGraph />
    </div>
  );
}
