import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/pages/components/page-header";
import { GeneralTab } from "./components/tabs/general-tab";
import { ContentTab } from "./components/tabs/content-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { FooterTab } from "./components/tabs/footer-tab";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function FuelCalculator() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["fuel-calculator"] ?? false,
  );

  return (
    <div>
      <PageHeader
        isActive={isActive}
        id="fuel-calculator"
        title="Fuel Calculator"
        description="Fuel calculator keeps track of your average fuel usage per lap and calculate the refuel amount. In a team session, you can keep track of your teammates fuel level, if they also use ioverlay."
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
