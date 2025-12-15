import type { TelemetryVar } from "@/lib/constants/telemetry-vars";

import { Checkbox } from "@/modules/components/ui/checkbox";
import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";

interface OptionsProps {
  options: { title: string }[];
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
              // onCheckedChange={(e) => toggleVar(option.title, e)}
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
