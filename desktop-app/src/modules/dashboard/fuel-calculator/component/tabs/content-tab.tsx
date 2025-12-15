import { DisplayOptions } from "@/modules/dashboard/components/display-options";

export const ContentTab = () => {
  const options = [
    { title: "Average last 5 laps" },
    { title: "Last lap" },
    { title: "Qualy lap" },
    { title: "Average 10 laps" },
    { title: "Minimal usage in session" },
    { title: "Maximum usage in session" },
  ];

  return <DisplayOptions options={options} toggleVar={() => {}} />;
};
