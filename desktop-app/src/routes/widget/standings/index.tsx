import { createFileRoute } from "@tanstack/react-router";
import { Standings } from "@/routes/widget/components/widget/standings";

export const Route = createFileRoute(`/widget/standings/`)({
  component: StandingsWidget,
});

export default function StandingsWidget() {
  return (
    <div>
      <Standings />
    </div>
  );
}
