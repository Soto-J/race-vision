import { cn } from "@/lib/utils";

import { Switch } from "@/modules/components/ui/switch";

interface PageHeaderProps {
  id: string;
  page: string;
  description: string;
  pageIsActive: boolean;
  setPageActive: (page: string, isActive: boolean) => void;
}

export const PageHeader = ({
  id,
  page,
  description,
  pageIsActive,
  setPageActive,
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
            onCheckedChange={(checked) => {
              console.log("Checked", checked);
              console.log("Page", page);
              setPageActive(page, checked);
            }}
          />
        </div>
      </div>

      <p className="text-muted-foreground text-sm leading-relaxed">
        {description}
      </p>
    </div>
  );
};
