import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";

interface OptionsProps {
  options: { title: string }[];
}

export const DisplayOptions = ({ options }: OptionsProps) => {
  return (
    <div className="space-y-2">
      {options.map((option) => (
        <div key={option.title} className="flex justify-between gap-x-2">
          <div className="flex gap-x-2">
            <Switch key={option.title} id={`${option.title}-toggle`} />
            <div>{option.title}</div>
          </div>

          <div className="flex items-center gap-x-4">
            <div className="flex items-center justify-center gap-x-4">
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
