import { Label } from "@/modules/components/ui/label";
import { Switch } from "@/modules/components/ui/switch";
import { PageConfig } from "@/hooks/store/types";

interface GeneralTabProps {
  page: string;
  settings: PageConfig;
  updateSettings: (
    page: string,
    setting: string,
    isActive: boolean,
  ) => Promise<void>;
}

export const GeneralTab = ({
  page,
  settings,
  updateSettings,
}: GeneralTabProps) => {
  return (
    <div className="flex justify-around">
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings["inCar"].displayIn.inCar}
          onCheckedChange={() => updateSettings(page, "inCar")}
        />
        <Label>In car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.displayIn.outOfCar}
          onCheckedChange={() => updateSettings(page, "outOfCar")}
        />
        <Label>Out of car</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.displayIn.spotting}
          onCheckedChange={() => updateSettings(page, "spotting")}
        />
        <Label>Spotting</Label>
      </div>
      <div className="flex items-center gap-x-2">
        <Switch
          checked={settings.displayIn.inGarage}
          onCheckedChange={() => updateSettings(page, "inGarage")}
        />
        <Label>In garage</Label>
      </div>
    </div>
  );
};
