import { Outlet } from "react-router-dom";

import { useTelemetryListener } from "./hooks/listeners/use-telemetry-listener";
import { useLoadWidgets } from "./hooks/use-load-widgets";
import { usePersistWidget } from "./hooks/use-persist-widget";
import { useEditModeListener } from "./hooks/listeners/use-edit-mode-listener";

import { SidebarProvider, SidebarTrigger } from "./components/sidebar";
import { AppSidebar } from "./components/sidebar/app-sidebar";
import { ThemeProvider } from "./components/theme-provider";
import { ThemeToggle } from "./components/theme-toggle";

export default function AppLayout() {
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
        </main>
      </SidebarProvider>
    </ThemeProvider>
  );
}
