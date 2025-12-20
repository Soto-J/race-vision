import { cn } from "@/lib/utils";
import { Activity, useCallback } from "react";
import { PageHeader } from "../../components/page-header";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/modules/components/ui/tabs";

import { TelemetryVar } from "@/lib/constants/telemetry-vars";
import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { GeneralTab } from "../component/tabs/general-tab";
import { ContentTab } from "../component/tabs/content-tab";
import { HeaderTab } from "../component/tabs/header-tab";
import { FooterTab } from "../component/tabs/footer-tab";

const INPUTS_TABS = ["general", "content", "header", "footer"] as const;

interface InputsViewProps {
  title: string;
}

export const InputsView = ({ title }: InputsViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;

  const toggleInputsVar = useCallback(
    (varName: TelemetryVar, enabled: boolean) =>
      toggleVar(title, varName, enabled),
    [toggleVar],
  );

  return (
    <div>
      <PageHeader
        id={title}
        title={title}
        description="Show your inputs in this window, you can even make this visible in a graph."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <Tabs>
          <TabsList
            defaultValue="general"
            className="bg-muted mx-auto mb-4 flex gap-2 rounded-full p-1"
          >
            {INPUTS_TABS.map((tab) => (
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
};
