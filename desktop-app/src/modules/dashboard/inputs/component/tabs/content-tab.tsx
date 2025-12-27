import { DisplayOptions } from "@/modules/dashboard/components/display-options";

import { Section } from "@/hooks/store/types";

interface ContentTabProps {
  settings: Section;
  updateSetting: (setting: string) => Promise<void>;
}

export const ContentTab = ({ settings, updateSetting }: ContentTabProps) => {
  const options = [
    { title: "Rev lights", key: "revLights" as const },
    { title: "Gears and speed", key: "gearsAndSpeed" as const },
    { title: "Input graph", key: "inputsGraph" as const },
    { title: "ABS activation", key: "ABSActivation" as const },
    { title: "Input bars", key: "inputBars" as const },
    { title: "Boost / ERS", key: "boostERS" as const },
    { title: "Corner speed", key: "cornerSpeed" as const },
  ];

  return (
    <DisplayOptions
      settings={settings}
      options={options}
      updateSetting={updateSetting}
    />
  );
};
