import { Tabs, TabsContent, TabsList } from "@/components/ui/tabs";
import { TabsTrigger } from "@radix-ui/react-tabs";
import { PageHeader } from "../components/page-header";

export default function Inputs() {
  return (
    <div className="py-4">
      <PageHeader
        id="inputs"
        title="Inputs"
        description="Show your inputs in this window, you can even make this visible in a graph."
        vars={[]}
      />

      <Tabs>
        <TabsList>
          <TabsTrigger value="general">General</TabsTrigger>
          <TabsTrigger value="content">Content</TabsTrigger>
          <TabsTrigger value="header">Header</TabsTrigger>
          <TabsTrigger value="footer">footer</TabsTrigger>
        </TabsList>

        <TabsContent value="general"></TabsContent>
        <TabsContent value="content"></TabsContent>
        <TabsContent value="header"></TabsContent>
        <TabsContent value="footer"></TabsContent>
      </Tabs>
    </div>
  );
}
