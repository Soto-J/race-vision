import { cn } from "@/lib/utils";

import { Switch } from "@/modules/components/ui/switch";

interface PageHeaderProps {
  id: string;
  page: string;
  description: string;
  pageIsActive: boolean;
  onToggleActive: (page: string) => void;
}

export const PageHeader = ({
  id,
  page,
  description,
  pageIsActive,
  onToggleActive,
}: PageHeaderProps) => {
  return (
    <div
      className={cn(
        "mb-6 space-y-3 border-b py-6",
        pageIsActive ? "" : "opacity-40",
      )}
    >
      <div className="flex items-start justify-between">
        <h2 className="text-3xl font-semibold tracking-tight capitalize">
          {page}
        </h2>

        <div className="flex items-center gap-2">
          <label htmlFor={`${id}-toggle`} className="text-sm">
            Active
          </label>
          <Switch
            className=""
            id={`${id}-toggle`}
            checked={pageIsActive}
            onCheckedChange={() => onToggleActive(page)}
          />
        </div>
      </div>

      <p className="text-muted-foreground text-sm leading-relaxed">
        {description}
      </p>
    </div>
  );
};
