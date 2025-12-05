import { useTelemetry } from "@/hooks/useTelemetry";
import OverlayPageLayout from "./layout";

interface OverlayPageProps {
  children: React.ReactNode;
}
export default function OverlayPage() {
  useTelemetry();

  return <div>OverlayWindow</div>;
}
