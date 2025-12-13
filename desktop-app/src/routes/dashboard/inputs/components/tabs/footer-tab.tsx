import type { TelemetryVar } from "@/lib/constants/telemetry-vars";

import { DisplayOptions } from "@/routes/dashboard/components/display-options";
import { DISPLAY_OPTIONS } from "@/routes/constants";

interface HeaderTabProps {
  toggleVar: (varName: TelemetryVar, enabled: boolean) => void;
}

export const FooterTab = ({ toggleVar }: HeaderTabProps) => {
  // TODO!
  return <DisplayOptions options={DISPLAY_OPTIONS} toggleVar={toggleVar} />;
};
