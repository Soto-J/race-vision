import { DisplayOptions } from "@/modules/dashboard/components/display-options";

export const ClassHeader = () => {
  const options = [
    { title: "Class name" },
    { title: "Drivers in class" },
    { title: "Class SOF" },
  ];

  return <DisplayOptions options={options} toggleVar={() => {}} />;
};
