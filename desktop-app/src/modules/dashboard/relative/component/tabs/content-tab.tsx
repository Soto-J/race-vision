import { DisplayOptions } from "@/modules/dashboard/components/display-options";

export const ContentTab = () => {
  const options = [
    { title: "Position" },
    { title: "Class badge color" },
    { title: "Driver name" },
    { title: "License" },
    { title: "iRating" },
    { title: "Relative time" },
    { title: "Team name" },
    { title: "Car brand" },
    { title: "Intervals" },
    { title: "Gap" },
    { title: "Fastest lap time" },
    { title: "Last lap time" },
    { title: "Lap delta" },
    { title: "Pitstop status" },
    { title: "Time in pitlane" },
    { title: "Time in pitbox" },
    { title: "Current stint length" },
    { title: "Positions gained" },
    { title: "Joker lap" },
    { title: "Tire compound" },
    { title: "Driver tags" },
    { title: "Black flag" },
    { title: "AVG 5 laps" },
    { title: "AVG 10 laps" },
    { title: "Push to pass" },
    { title: "Country flag" },
  ];

  return <DisplayOptions options={options} toggleVar={() => {}} />;
};
