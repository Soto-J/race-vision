import { Outlet } from "react-router-dom";

import { SidebarProvider, SidebarTrigger } from "./components/sidebar";
import { AppSidebar } from "./components/sidebar/app-sidebar";

export default function AppLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />

      <main className="h-screen w-screen">
        <SidebarTrigger />
        <Outlet />
      </main>
    </SidebarProvider>
  );
}
