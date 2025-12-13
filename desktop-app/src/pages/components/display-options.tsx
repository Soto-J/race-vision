import type { TelemetryVar } from "@/lib/constants/telemetry-vars";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";

interface OptionsProps {
  options: { title: TelemetryVar }[];
  toggleVar: (varName: TelemetryVar, enabled: boolean) => void;
}

export const DisplayOptions = ({ options, toggleVar }: OptionsProps) => {
  return (
    <div className="space-y-2">
      {options.map((option) => (
        <div key={option.title} className="flex justify-between gap-x-2">
          <div className="flex items-center gap-x-2">
            <Switch
              key={option.title}
              id={`${option.title}-toggle`}
              onCheckedChange={(e) => toggleVar(option.title, e)}
            />
            <div>{option.title}</div>
          </div>

          <div className="flex items-center gap-x-4">
            <div className="flex items-center justify-center gap-x-2">
              <Checkbox />
              <Label>Race</Label>
            </div>
            <div className="flex items-center justify-center gap-x-2">
              <Checkbox />
              <Label>Qualy</Label>
            </div>
            <div className="flex items-center justify-center gap-x-2">
              <Checkbox />
              <Label>Practice</Label>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
