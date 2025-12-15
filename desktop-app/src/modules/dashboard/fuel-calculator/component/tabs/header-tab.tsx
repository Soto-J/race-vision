import { DisplayOptions } from "@/modules/dashboard/components/display-options";

import { DISPLAY_OPTIONS } from "@/lib/constants";

export const HeaderTab = () => {
  return <DisplayOptions options={DISPLAY_OPTIONS} toggleVar={() => {}} />;
};
