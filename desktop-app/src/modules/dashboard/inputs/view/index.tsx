import { Activity } from "react";
import { cn } from "@/lib/utils";

import { PageConfig } from "@/hooks/store/types";

import { PageHeader } from "@/modules/dashboard/components/page-header";
import { GeneralTab } from "@/modules/dashboard/inputs/component/tabs/general-tab";
import { ContentTab } from "@/modules/dashboard/inputs/component/tabs/content-tab";
import { HeaderTab } from "@/modules/dashboard/inputs/component/tabs/header-tab";
import { FooterTab } from "@/modules/dashboard/inputs/component/tabs/footer-tab";

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/modules/components/ui/tabs";

interface InputsViewProps {
  page: string;
  settings: PageConfig;
  setPageActive: (page: string, isActive: boolean) => Promise<void>;
  updateSettings: (setting: string, isActive: boolean) => Promise<void>;
}

export const InputsView = ({
  page,
  settings,
  setPageActive,
}: InputsViewProps) => {
  return (
    <div>
      <PageHeader
        id={page}
        page={page}
        description="Show your inputs in this window, you can even make this visible in a graph."
        pageIsActive={settings.isActive}
        setPageActive={setPageActive}
      />

      <Activity mode={settings.isActive ? "visible" : "hidden"}>
        <Tabs>
          <TabsList
            defaultValue="general"
            className="bg-muted mx-auto mb-4 flex gap-2 rounded-full p-1"
          >
            {["general", "content", "header", "footer"].map((tab) => (
              <TabsTrigger
                key={tab}
                value={tab}
                className={cn(
                  "rounded-full px-4 py-2 text-sm font-medium capitalize",
                  "transition data-[state=inactive]:opacity-60",
                )}
              >
                {tab}
              </TabsTrigger>
            ))}
          </TabsList>

          <TabsContent value="general">
            <GeneralTab
              general={settings.general}
              updateSettings={updateSettings}
            />
          </TabsContent>
          <TabsContent value="content">
            <ContentTab
              settings={settings.content}
              updateSettings={updateSettings}
            />
          </TabsContent>
          <TabsContent value="header">
            <HeaderTab
              settings={settings.header}
              updateSettings={updateSettings}
            />
          </TabsContent>
          <TabsContent value="footer">
            <FooterTab
              settings={settings.footer}
              updateSettings={updateSettings}
            />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
};
