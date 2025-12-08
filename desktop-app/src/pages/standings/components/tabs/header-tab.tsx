import { DisplayOptions } from "@/pages/components/display-options";


export const HeaderTab = () => {
  const displayOptions = [
    {title: "Session name"},
    {title: "Laps remaining"},
    {title: "Track name"},
    {title: "SOF"},
    {title: "Event type"},
    {title: "Local time (24h)"},
    {title: "Local time (am/pm)"},
    {title: "Local time (am/pm)"},
  ]
  return <DisplayOptions options={displayOptions} />;
};
