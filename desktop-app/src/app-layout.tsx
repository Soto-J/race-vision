import { Outlet } from "react-router-dom";

import { useTelemetry } from "./hooks/use-telemetry";

import { useLoadWidgets } from "./hooks/use-load-widgets";
import { usePersistWidget } from "./hooks/use-persist-widget";

import { SidebarProvider, SidebarTrigger } from "./components/sidebar";
import { AppSidebar } from "./components/sidebar/app-sidebar";
import { ThemeProvider } from "./components/theme-provider";
import { ThemeToggle } from "./components/theme-toggle";
import { useToggleEditMode } from "./hooks/use-toggle-edit-mode";

export default function AppLayout() {
  useTelemetry();

  useToggleEditMode();
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
