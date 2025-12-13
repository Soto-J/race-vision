import { createFileRoute } from "@tanstack/react-router";

import { PedalGraph } from "@/routes/widget/components/widget/pedal-graph";

export const Route = createFileRoute(`/widget/relative/`)({
  component: RelativeWidget,
});

export default function RelativeWidget() {
  return (
    <div data-tauri-drag-region>
      <PedalGraph />
    </div>
  );
}
