import { Activity } from "react";
import { cn } from "@/lib/utils";
import { z } from "zod";

import type { InputsSettings } from "../types";

import { useUpdateSettings } from "@/hooks/store/use-update-settings";

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
  title: string;
  settings: InputsSettings;
  schema: z.ZodSchema;
}

export const InputsView = ({ title, settings, schema }: InputsViewProps) => {
  const updateSettings = useUpdateSettings(title, schema);

  const togglePage = () => {
    updateSettings.mutate({
      ...settings,
      settings: {
        isActive: !settings.isActive,
      },
    });
  };

  type ContentKey = keyof typeof settings.content;

  const toggleFeature = (path: ContentKey, value: boolean, id: string) => {
    updateSettings.mutate({
      ...settings,
      settings: {
        [path]: {
          ...settings.[path as keyof typeof settings.[path]],
          [id]: value
        },
      },
    });
  };

  return (
    <div>
      <PageHeader
        id={title}
        title={title}
        description="Show your inputs in this window, you can even make this visible in a graph."
        pageIsActive={settings.isActive}
        togglePage={togglePage}
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
            <GeneralTab />
          </TabsContent>
          <TabsContent value="content">
            <ContentTab />
          </TabsContent>
          <TabsContent value="header">
            <HeaderTab toggleVar={toggleFeature} />
          </TabsContent>
          <TabsContent value="footer">
            <FooterTab toggleVar={toggleFeature} />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
};
