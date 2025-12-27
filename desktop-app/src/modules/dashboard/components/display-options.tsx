import { Section } from "@/hooks/store/types";
import { Checkbox } from "@/modules/components/ui/checkbox";
import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";

interface FeatureSettings {
  isActive: boolean;
  displayIn: {
    race: boolean;
    qualy: boolean;
    practice: boolean;
  };
}

interface OptionsProps {
  settings: Section;
  options: { title: string; key: keyof TSettings }[];
  updateSetting: (setting: string) => void;
}

export const DisplayOptions = <
  TSettings extends Record<string, FeatureSettings>,
>({
  settings,
  options,
  updateSetting,
}: OptionsProps<TSettings>) => {
  return (
    <div className="space-y-2">
      {options.map((option) => {
        const featureSettings = settings[option.key];

        return (
          <div
            key={option.key as string}
            className="flex justify-between gap-x-2"
          >
            <div className="flex items-center gap-x-2">
              <Switch
                id={`${option.key as string}-toggle`}
                checked={featureSettings.isActive}
                onCheckedChange={() => updateSetting(option.key)}
              />
              <div>{option.title}</div>
            </div>

            <div className="flex items-center gap-x-4">
              <div className="flex items-center justify-center gap-x-2">
                <Checkbox checked={featureSettings.displayIn.race} disabled />
                <Label>Race</Label>
              </div>
              <div className="flex items-center justify-center gap-x-2">
                <Checkbox checked={featureSettings.displayIn.qualy} disabled />
                <Label>Qualy</Label>
              </div>
              <div className="flex items-center justify-center gap-x-2">
                <Checkbox
                  checked={featureSettings.displayIn.practice}
                  disabled
                />
                <Label>Practice</Label>
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
};
