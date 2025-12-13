import { createFileRoute } from "@tanstack/react-router";

import { PedalGraph } from "./components/widget/pedal-graph";
import { Relative } from "./components/widget/relative";
import { Standings } from "./components/widget/standings";
import { TrackMap } from "./components/widget/track-map";

export const Route = createFileRoute("/widget/")({
  component: WidgetsPage,
});

interface WidgetsPageProps {
  widgetId: string;
}

export default function WidgetsPage() {
  return null;
}
