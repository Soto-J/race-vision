import { Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import { useTelemetryListener } from "@/hooks/listeners/use-telemetry-listener";

import { ThemeToggle } from "@/modules/components/theme-toggle";

import { AppSidebar } from "@/modules/dashboard/components/sidebar/app-sidebar";
import {
  SidebarProvider,
  SidebarTrigger,
} from "@/modules/dashboard/components/sidebar";

export default function DashboardLayout() {
  useTelemetryListener();

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
