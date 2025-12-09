import { useTelemetry } from "@/hooks/useTelemetry";

import Standings from "@/pages/standings";
import TrackMap from "@/pages/track-map";
import Relative from "@/pages/relative";

import { PedalGraph } from "./components/widget/pedal-graph";
import { DraggableWidget } from "./components/draggable-widget";

export default function OverlayPage() {
  useTelemetry();

  const params = new URLSearchParams(window.location.search);
  const widget = params.get("widget");

  switch (widget) {
    case "pedals":
      return (
        <DraggableWidget id="pedal-graph" width={800} height={800}>
          <PedalGraph />
        </DraggableWidget>
      );
    case "standings":
      return (
        <DraggableWidget id="standings-widget" width={300} height={600}>
          <Standings />
        </DraggableWidget>
      );
    case "trackmap":
      return (
        <DraggableWidget id="trackmap-widget" width={500} height={500}>
          <TrackMap />
        </DraggableWidget>
      );
    case "relative":
      return (
        <DraggableWidget id="relative-widget" width={300} height={700}>
          <Relative />
        </DraggableWidget>
      );
    default:
      null;
  }
}
