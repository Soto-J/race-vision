import { cn } from "@/lib/utils";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { Switch } from "@/components/ui/switch";

interface PageHeaderProps {
  id: string;
  title: string;
  description: string;
  isActive: boolean;
  vars: string[]; // list of telemetry variables this page needs
}

export const PageHeader = ({
  id,
  title,
  description,
  isActive,
  vars,
}: PageHeaderProps) => {
  const toggle = useTelemetryStore((s) => s.toggleSetting);

  return (
    <div
      className={cn(
        "mb-6 space-y-3 border-b py-6",
        isActive ? "" : "opacity-40",
      )}
    >
      <div className="flex items-start justify-between">
        <h2 className="text-3xl font-semibold tracking-tight">{title}</h2>

        <div className="flex items-center gap-2">
          <label htmlFor={`${id}-toggle`} className="text-sm">
            Active
          </label>
          <Switch
            className=""
            id={`${id}-toggle`}
            checked={isActive}
            onCheckedChange={(value) => toggle(id, value, vars)}
          />
        </div>
      </div>

      <p className="text-muted-foreground text-sm leading-relaxed">
        {description}
      </p>
    </div>
  );
};
