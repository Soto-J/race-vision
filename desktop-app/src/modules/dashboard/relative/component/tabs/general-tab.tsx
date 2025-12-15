import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";

export const GeneralTab = () => {
  return (
    <div className="flex justify-around">
      <div className="flex items-center gap-x-2">
        <Switch />
        <Label>In car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch />
        <Label>Out of car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch />
        <Label>Spotting</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch />
        <Label>In garage</Label>
      </div>
    </div>
  );
};
