import { Activity } from "react";

import { useTelemetryStore } from "@/hooks/useTelemetryStore";

import { PageHeader } from "../components/page-header";
import { ContentTab } from "./components/tabs/content-tab";
import { GeneralTab } from "./components/tabs/general-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { ClassHeader } from "./components/tabs/class-header";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function Standings() {
  const isActive = useTelemetryStore(
    (state) => state.isActive["standings"] ?? false,
  );

  return (
    <div>
      <PageHeader
        id="standings"
        title="Standings"
        description="This overlay shows the other drivers position and information that is relevant during a race, qualy or practice session. The info is customizable in the content section."
        vars={[]}
        isActive={false}
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
}
