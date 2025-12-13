import { createFileRoute } from "@tanstack/react-router";
import { TrackMap } from "@/routes/widget/components/widget/track-map";

export const Route = createFileRoute(`/widget/track-map/`)({
  component: TrackMapWidget,
});

export default function TrackMapWidget() {
  return (
    <div>
      <TrackMap />
    </div>
  );
}
