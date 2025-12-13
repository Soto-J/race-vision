import { Activity } from "react";
import { createFileRoute } from "@tanstack/react-router";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/routes/dashboard/components/page-header";
import { GeneralTab } from "./components/tabs/general-tab";
import { ContentTab } from "./components/tabs/content-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { FooterTab } from "./components/tabs/footer-tab";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const PAGE_TITLE = "fuel-calculator";

export const Route = createFileRoute(`/dashboard/${PAGE_TITLE}/`)({
  component: FuelCalculator,
});

export default function FuelCalculator() {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[PAGE_TITLE] ?? 0;

  return (
    <div>
      <PageHeader
        id={PAGE_TITLE}
        title={PAGE_TITLE.replace("-", " ")}
        description="Fuel calculator keeps track of your average fuel usage per lap and calculate the refuel amount. In a team session, you can keep track of your teammates fuel level, if they also use ioverlay."
        pageIsActive={isActive}
        togglePage={togglePage}
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
