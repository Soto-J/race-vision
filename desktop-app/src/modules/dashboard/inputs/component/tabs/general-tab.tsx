import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";
import { GeneralSection } from "@/hooks/store/types";

interface GeneralTabProps {
  general: GeneralSection;
  updateSettings: (setting: string, isActive: boolean) => Promise<void>;
}

export const GeneralTab = ({ general, updateSettings }: GeneralTabProps) => {
  return (
    <div className="flex justify-around">
      <div className="flex items-center gap-x-2">
        <Switch
          checked={general.inCar.isActive}
          onCheckedChange={(checked) => updateSettings("inCar", checked)}
        />
        <Label>In car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={general.outOfCar.isActive}
          onCheckedChange={(checked) => updateSettings("outOfCar", checked)}
        />
        <Label>Out of car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={general.spotting.isActive}
          onCheckedChange={(checked) => updateSettings("spotting", checked)}
        />
        <Label>Spotting</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={general.inGarage.isActive}
          onCheckedChange={(checked) => updateSettings("inGarage", checked)}
        />
        <Label>In garage</Label>
      </div>
    </div>
  );
};
