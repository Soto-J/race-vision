import { PageHeader } from "../components/page-header";
import { GeneralTab } from "./components/tabs/general-tab";
import { ContentTab } from "./components/tabs/content-tab";
import { HeaderTab } from "./components/tabs/header-tab";
import { FooterTab } from "./components/tabs/footer-tab";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useTelemetryStore } from "@/hooks/useTelemetryStore";
import { cn } from "@/lib/utils";

export default function Inputs() {
  const isActive = useTelemetryStore((s) => s.isActive["inputs"] ?? false);

  return (
    <div className="">
      <PageHeader
        id="inputs"
        title="Inputs"
        isActive={isActive}
        description="Show your inputs in this window, you can even make this visible in a graph."
        vars={[]}
      />

      <Tabs className={cn(isActive ? "" : "opacity-0")}>
        <TabsList className="mx-auto gap-x-2">
          <TabsTrigger value="general">General</TabsTrigger>
          <TabsTrigger value="content">Content</TabsTrigger>
          <TabsTrigger value="header">Header</TabsTrigger>
          <TabsTrigger value="footer">footer</TabsTrigger>
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
    </div>
  );
}
