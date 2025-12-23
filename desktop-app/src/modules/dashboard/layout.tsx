import { Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import { useTelemetryListener } from "@/hooks/listeners/use-telemetry-listener";

import { ThemeToggle } from "@/modules/components/theme-toggle";

import { AppSidebar } from "@/modules/dashboard/components/sidebar/app-sidebar";
import {
  SidebarProvider,
  SidebarTrigger,
} from "@/modules/dashboard/components/sidebar";
import { usePageSettingsStore } from "@/hooks/store/use-page-store";
import { useEffect } from "react";

export default function DashboardLayout() {
  useTelemetryListener();
  const { loadPages } = usePageSettingsStore();

  useEffect(() => {
    loadPages();
  }, []);
  
  return (
    <SidebarProvider>
      <AppSidebar />

      <main className="w-full p-4">
        <div className="flex justify-between">
          <SidebarTrigger />
          <ThemeToggle className="h-6 w-6" />
        </div>

        <Outlet />
        <TanStackRouterDevtools />
      </main>
    </SidebarProvider>
  );
}
