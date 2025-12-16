import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";
import { GeneralTab } from "@/modules/dashboard/fuel-calculator/component/tabs/general-tab";
import { ContentTab } from "@/modules/dashboard/fuel-calculator/component/tabs/content-tab";
import { HeaderTab } from "@/modules/dashboard/fuel-calculator/component/tabs/header-tab";
import { FooterTab } from "@/modules/dashboard/fuel-calculator/component/tabs/footer-tab";

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/modules/components/ui/tabs";

interface FuelCalculatorViewProps {
  title: string;
}
export const FuelCalculatorView = ({ title }: FuelCalculatorViewProps) => {
  const { pageIsActive, togglePage } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? 0;
  return (
    <div>
      <PageHeader
        id={title}
        title={title.replace("-", " ")}
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
};
