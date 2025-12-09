import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "@/pages/components/page-header";
import { GeneralTab } from "./components/tabs/general-tab";
import { ContentTab } from "./components/tabs/content-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { FooterTab } from "./components/tabs/footer-tab";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function Inputs() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["inputs"] ?? false,
  );

  const tabs = ["general", "content", "header", "footer"] as const;

  return (
    <div className="">
      <PageHeader
        isActive={isActive}
        id="inputs"
        title="Inputs"
        description="Show your inputs in this window, you can even make this visible in a graph."
        vars={[]}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <Tabs defaultValue="general">
          <TabsList className="bg-muted mx-auto mb-4 flex gap-2 rounded-full p-1">
            {tabs.map((tab) => (
              <TabsTrigger
                key={tab}
                value={tab}
                className="data-[state=active]:bg-primary data-[state=active]:text-primary rounded-full px-4 py-2 text-sm font-medium transition data-[state=inactive]:opacity-60"
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
            <HeaderTab />
          </TabsContent>
          <TabsContent value="footer">
            <FooterTab />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
}
