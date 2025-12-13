import { DisplayOptions } from "@/routes/dashboard/components/display-options";
import { DISPLAY_OPTIONS } from "@/routes/constants";

export const FooterTab = () => {
  return <DisplayOptions options={DISPLAY_OPTIONS} />;
};
