import { createFileRoute } from "@tanstack/react-router";

import { Standings } from "@/modules/widget/components/standings";

export const Route = createFileRoute(`/widget/standings/`)({
  component: StandingsWidget,
});

export default function StandingsWidget() {
  return (
    <div data-tauri-drag-region>
      <Standings />
    </div>
  );
}
