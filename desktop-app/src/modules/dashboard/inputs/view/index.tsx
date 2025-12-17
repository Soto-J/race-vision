import { Activity } from "react";
import { cn } from "@/lib/utils";
import { z } from "zod";

import type { InputsSettings } from "../types";

import { useUpdateSettings } from "@/hooks/settings/use-update-settings";
import {
  toggleFeature,
  toggleGeneralFeature,
  type FeatureKey,
  type GeneralFeatureKey,
} from "@/hooks/settings/helper";

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

  const onToggleActive = () => {
    updateSettings.mutate({
      ...settings,
      isActive: !settings.isActive,
    });
  };

  const onToggleGeneral = (feature: GeneralFeatureKey) => {
    updateSettings.mutate(toggleGeneralFeature(settings, feature));
  };

  const onToggleContent = (feature: FeatureKey<"content">) => {
    updateSettings.mutate(toggleFeature(settings, "content", feature));
  };

  const onToggleHeader = (feature: FeatureKey<"header">) => {
    updateSettings.mutate(toggleFeature(settings, "header", feature));
  };

  const onToggleFooter = (feature: FeatureKey<"footer">) => {
    updateSettings.mutate(toggleFeature(settings, "footer", feature));
  };

  return (
    <div>
      <PageHeader
        id={title}
        title={title}
        description="Show your inputs in this window, you can even make this visible in a graph."
        pageIsActive={settings.isActive}
        togglePage={onToggleActive}
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
              settings={settings.general}
              toggleFeature={onToggleGeneral}
            />
          </TabsContent>
          <TabsContent value="content">
            <ContentTab
              settings={settings.content}
              toggleFeature={onToggleContent}
            />
          </TabsContent>
          <TabsContent value="header">
            <HeaderTab
              settings={settings.header}
              toggleFeature={onToggleHeader}
            />
          </TabsContent>
          <TabsContent value="footer">
            <FooterTab
              settings={settings.footer}
              toggleFeature={onToggleFooter}
            />
          </TabsContent>
        </Tabs>
      </Activity>
    </div>
  );
};
