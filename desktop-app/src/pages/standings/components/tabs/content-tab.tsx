import { DisplayOptions } from "@/pages/components/display-options";

export const ContentTab = () => {
  const options = [
    { title: "Position" },
    { title: "Class badge color" },
    { title: "Car number" },
    { title: "Team name" },
    { title: "License" },
    { title: "iRating" },
    { title: "Car brand" },
    { title: "Interval" },
    { title: "Fastest lap time" },
    { title: "Gap" },
    { title: "Lap delta" },
    { title: "Positions gained" },
    { title: "Black flag" },
    { title: "Driver name" },
    { title: "Pitstop status" },
    { title: "Time in pitlane" },
    { title: "Relative time" },
    { title: "Time in pitbox" },
    { title: "Current stint length" },
    { title: "Joker lap" },
    { title: "Tire compound" },
    { title: "Driver tags" },
    { title: "AVG 5 laps" },
    { title: "AVG 10 laps" },
    { title: "Push to pass" },
    { title: "Country flag" },
  ];

  return <DisplayOptions options={options} />;
};
