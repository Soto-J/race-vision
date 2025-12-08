import { useTelemetry } from "@/hooks/useTelemetry";
import OverlayPageLayout from "./layout";
import { PedalGraph } from "./components/pedal-graph";

interface OverlayPageProps {
  children: React.ReactNode;
}
export default function OverlayPage() {
  useTelemetry();

  return (
    <div>
      OverlayWindow
      <div>
        <PedalGraph />
      </div>
    </div>
  );
}
