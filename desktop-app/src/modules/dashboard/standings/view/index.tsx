import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { PageHeader } from "@/modules/dashboard/components/page-header";
import { ContentTab } from "@/modules/dashboard/standings/component/tabs/content-tab";
import { GeneralTab } from "@/modules/dashboard/standings/component/tabs/general-tab";
import { HeaderTab } from "@/modules/dashboard/standings/component/tabs/header-tab";
import { ClassHeader } from "@/modules/dashboard/standings/component/tabs/class-header";

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/modules/components/ui/tabs";

interface StandingsViewProps {
  title: string;
}

export const StandingsView = ({ title }: StandingsViewProps) => {
  const { pageIsActive, togglePage, toggleVar } = useTelemetryStore();

  const isActive = pageIsActive[title] ?? false;
  return (
    <div>
      <PageHeader
        id={title}
        title={title}
        description="This overlay shows the other drivers position and information that is relevant during a race, qualy or practice session. The info is customizable in the content section."
        pageIsActive={isActive}
        togglePage={togglePage}
      />

      <Activity mode={isActive ? "visible" : "hidden"}>
        <Tabs defaultValue="general">
          <TabsList className="mx-auto mb-4 gap-x-2">
            <TabsTrigger value="general">General</TabsTrigger>
            <TabsTrigger value="content">Content</TabsTrigger>
            <TabsTrigger value="class-header">Class header</TabsTrigger>
            <TabsTrigger value="header">Header</TabsTrigger>
          </TabsList>

          <TabsContent value="general">
            <GeneralTab />
          </TabsContent>
          <TabsContent value="content">
            <ContentTab />
          </TabsContent>
          <TabsContent value="class-header">
            <ClassHeader />
          </TabsContent>
          <TabsContent value="header">
            <HeaderTab />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
};
