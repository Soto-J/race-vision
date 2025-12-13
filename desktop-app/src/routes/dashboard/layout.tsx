import { Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import { useEditModeListener } from "@/hooks/listeners/use-edit-mode-listener";
import { useTelemetryListener } from "@/hooks/listeners/use-telemetry-listener";
import { useLoadWidgets } from "@/hooks/use-load-widgets";
import { usePersistWidget } from "@/hooks/use-persist-widget";

import { SidebarProvider, SidebarTrigger } from "./components/sidebar";
import { AppSidebar } from "./components/sidebar/app-sidebar";
import { ThemeToggle } from "@/components/theme-toggle";

export default function DashboardLayout() {
  useTelemetryListener();
  useEditModeListener();

  // useLoadWidgets();
  usePersistWidget();

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
