import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";
import type { GeneralFeatureKey } from "@/hooks/settings/helper";
import type { InputsSettings } from "../../types";

interface GeneralTabProps {
  settings: InputsSettings["general"];
  toggleFeature: (feature: GeneralFeatureKey) => void;
}

export const GeneralTab = ({ settings, toggleFeature }: GeneralTabProps) => {
  return (
    <div className="flex justify-around">
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.showOverlayWhen.inCar}
          onCheckedChange={() => toggleFeature("inCar")}
        />
        <Label>In car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.showOverlayWhen.outOfCar}
          onCheckedChange={() => toggleFeature("outOfCar")}
        />
        <Label>Out of car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.showOverlayWhen.spotting}
          onCheckedChange={() => toggleFeature("spotting")}
        />
        <Label>Spotting</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.showOverlayWhen.inGarage}
          onCheckedChange={() => toggleFeature("inGarage")}
        />
        <Label>In garage</Label>
      </div>
    </div>
  );
};
