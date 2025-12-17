import { DisplayOptions } from "@/modules/dashboard/components/display-options";
import type { FeatureKey } from "@/hooks/settings/helper";
import type { InputsSettings } from "../../types";
import { useUpdateSettings } from "@/hooks/settings/use-update-settings";

interface ContentTabProps {
  settings: InputsSettings["content"];
  updateSettings: ReturnType<typeof useUpdateSettings>;
}

export const ContentTab = ({ settings, updateSettings }: ContentTabProps) => {
  const toggleFeature = (feature: FeatureKey<"content">) => {
    updateSettings.mutate((prev) => ({
      ...prev,
      content: {
        ...prev.content,
        [feature]: {
          ...prev.content[feature],
          isActive: !prev.content[feature].isActive,
        },
      },
    }));
  };

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
      toggleFeature={toggleFeature}
    />
  );
};
