import { DisplayOptions } from "@/pages/components/display-options";

export const ContentTab = () => {
  const options = [
    { title: "Rev lights" },
    { title: "Gears and speed" },
    { title: "Input graph" },
    { title: "ABS activation" },
    { title: "Input bars" },
    { title: "Steering wheel" },
    { title: "Boost / ERS" },
    { title: "Corner speed" },
  ];

  return <DisplayOptions options={options} />;
};
