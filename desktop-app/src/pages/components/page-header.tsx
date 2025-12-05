import { cn } from "@/lib/utils";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { Switch } from "@/components/ui/switch";

interface PageHeaderProps {
  id: string;
  title: string;
  description: string;
  vars: string[]; // list of telemetry variables this page needs
}

export const PageHeader = ({
  id,
  title,
  description,
  vars,
}: PageHeaderProps) => {
  const isActive = useTelemetryStore((s) => s.isActive[id] ?? false);
  const toggle = useTelemetryStore((s) => s.toggleSetting);

  return (
    <div
      className={cn(
        "space-y-3 pb-6 mb-6 border-b",
        isActive ? "" : "opacity-40"
      )}
    >
      <div className="flex justify-between items-start">
        <h2 className="text-3xl font-semibold tracking-tight">{title}</h2>

        <div className="flex items-center gap-2">
          <label htmlFor={`${id}-toggle`} className="text-sm">
            Active
          </label>
          <Switch
            id={`${id}-toggle`}
            checked={isActive}
            onCheckedChange={(value) => toggle(id, value, vars)}
          />
        </div>
      </div>

      <p className="text-sm text-muted-foreground leading-relaxed">
        {description}
      </p>
    </div>
  );
};
