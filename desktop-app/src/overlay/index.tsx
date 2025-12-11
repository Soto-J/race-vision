import { PedalGraph } from "./components/widget/pedal-graph";
import { Relative } from "./components/widget/relative";
import { Standings } from "./components/widget/standings";
import { TrackMap } from "./components/widget/track-map";

interface OverlayPageProps {
  widgetId: string;
}

export default function OverlayPage({ widgetId }: OverlayPageProps) {
  switch (widgetId) {
    case "pedals":
      return <PedalGraph />;
    case "standings":
      return <Standings />;
    case "track-map":
      return <TrackMap />;
    case "relative":
      return <Relative />;
    default:
      null;
  }
}
