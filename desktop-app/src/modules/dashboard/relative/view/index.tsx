import { Activity, useCallback } from "react";
import { cn } from "@/lib/utils";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";
import { GeneralTab } from "@/modules/dashboard/relative/component/tabs/general-tab";
import { ContentTab } from "@/modules/dashboard/relative/component/tabs/content-tab";
import { HeaderTab } from "@/modules/dashboard/relative/component/tabs/header-tab";
import { FooterTab } from "@/modules/dashboard/relative/component/tabs/footer-tab";

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/modules/components/ui/tabs";

interface RelativeViewProps {
  title: string;
}

export const RelativeView = ({ title }: RelativeViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
  return (
    <div>
      <PageHeader
        id={title}
        title={title}
        description="The relative overlay show the other drivers around you."
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
