import type { TelemetryVar } from "@/lib/constants/telemetry-vars";

import { DisplayOptions } from "@/modules/dashboard/components/display-options";

import { DISPLAY_OPTIONS } from "@/lib/constants";

interface HeaderTabProps {
  toggleVar: (varName: TelemetryVar, enabled: boolean) => void;
}

export const HeaderTab = ({ toggleVar }: HeaderTabProps) => {
  // TODO!
  return <DisplayOptions options={DISPLAY_OPTIONS} toggleVar={toggleVar} />;
};
