import { Activity, useCallback } from "react";
import { cn } from "@/lib/utils";

import { TelemetryVar, TelemetryVars } from "@/lib/constants/telemetry-vars";
import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/pages/components/page-header";
import { GeneralTab } from "./components/tabs/general-tab";
import { ContentTab } from "./components/tabs/content-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { FooterTab } from "./components/tabs/footer-tab";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const INPUT_VARS: TelemetryVar[] = [
  TelemetryVars.THROTTLE,
  TelemetryVars.BRAKE,
  TelemetryVars.CLUTCH,
];

const tabs = ["general", "content", "header", "footer"] as const;

const PAGE_TITLE = "inputs";

export default function Inputs() {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();
  const isActive = pageIsActive[PAGE_TITLE] ?? false;

  const toggleInputsVar = useCallback(
    (varName: TelemetryVar, enabled: boolean) =>
      toggleVar(PAGE_TITLE, varName, enabled),
    [toggleVar],
  );

  return (
    <div>
      <PageHeader
        pageIsActive={isActive}
        togglePage={togglePage}
        id={PAGE_TITLE}
        title={PAGE_TITLE}
        description="Show your inputs in this window, you can even make this visible in a graph."
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <Tabs>
          <TabsList
            defaultValue="general"
            className="bg-muted mx-auto mb-4 flex gap-2 rounded-full p-1"
          >
            {tabs.map((tab) => (
              <TabsTrigger
                key={tab}
                value={tab}
                className={cn(
                  "rounded-full px-4 py-2 text-sm font-medium capitalize",
                  "transition data-[state=inactive]:opacity-60",
                )}
              >
                {tab}
              </TabsTrigger>
            ))}
          </TabsList>

          <TabsContent value="general">
            <GeneralTab />
          </TabsContent>
          <TabsContent value="content">
            <ContentTab />
          </TabsContent>
          <TabsContent value="header">
            <HeaderTab toggleVar={toggleInputsVar} />
          </TabsContent>
          <TabsContent value="footer">
            <FooterTab toggleVar={toggleInputsVar} />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
}
