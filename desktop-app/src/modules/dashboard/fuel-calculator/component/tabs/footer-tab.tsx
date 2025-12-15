import { DisplayOptions } from "@/modules/dashboard/components/display-options";

import { DISPLAY_OPTIONS } from "@/lib/constants";

export const FooterTab = () => {
  return <DisplayOptions options={DISPLAY_OPTIONS} toggleVar={() => {}} />;
};
