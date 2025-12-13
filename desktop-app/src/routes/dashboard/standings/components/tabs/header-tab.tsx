import { DisplayOptions } from "@/routes/dashboard/components/display-options";
import { DISPLAY_OPTIONS } from "@/routes/constants";

export const HeaderTab = () => {
  return <DisplayOptions options={DISPLAY_OPTIONS} />;
};
