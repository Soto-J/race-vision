import { Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import { useEditModeListener } from "@/hooks/listeners/use-edit-mode-listener";
import { useTelemetryListener } from "@/hooks/listeners/use-telemetry-listener";

import { ThemeToggle } from "@/modules/components/theme-toggle";
import {
  SidebarProvider,
  SidebarTrigger,
} from "@/modules/dashboard/components/sidebar";
import { AppSidebar } from "@/modules/dashboard/components/sidebar/app-sidebar";

export default function DashboardLayout() {
  useTelemetryListener();
  useEditModeListener();

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
