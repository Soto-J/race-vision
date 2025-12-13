import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";

import { useTelemetryListener } from "@/hooks/listeners/use-telemetry-listener";
import { useEditModeListener } from "@/hooks/listeners/use-edit-mode-listener";
import { useLoadWidgets } from "@/hooks/use-load-widgets";
import { usePersistWidget } from "@/hooks/use-persist-widget";

import { SidebarProvider, SidebarTrigger } from "@/components/sidebar";
import { AppSidebar } from "@/components/sidebar/app-sidebar";
import { ThemeProvider } from "@/components/theme-provider";
import { ThemeToggle } from "@/components/theme-toggle";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  useTelemetryListener();
  useEditModeListener();
  useLoadWidgets();
  usePersistWidget();

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
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
    </ThemeProvider>
  );
}
