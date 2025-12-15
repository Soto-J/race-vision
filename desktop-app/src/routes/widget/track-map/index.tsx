import { createFileRoute } from "@tanstack/react-router";

import { TrackMap } from "@/modules/widget/components/track-map";

export const Route = createFileRoute(`/widget/track-map/`)({
  component: TrackMapWidget,
});

export default function TrackMapWidget() {
  return (
    <div data-tauri-drag-region>
      <TrackMap />
    </div>
  );
}
