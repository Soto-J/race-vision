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
          <TabsList className="mx-auto mb-4 gap-x-2">
            <TabsTrigger value="general">General</TabsTrigger>
            <TabsTrigger value="content">Content</TabsTrigger>
            <TabsTrigger value="header">Header</TabsTrigger>
            <TabsTrigger value="footer">Footer</TabsTrigger>
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
