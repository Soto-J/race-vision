import { NavLink } from "react-router-dom";

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/sidebar";

export function AppSidebar() {
  const routes = [
    { title: "General", href: "/" },
    { title: "Standings", href: "standings" },
    { title: "Relative", href: "relative" },
    { title: "Fuel Calculator", href: "fuel-calculator" },
    { title: "Spotter", href: "spotter" },
    { title: "Pit Helper", href: "pit-helper" },
    { title: "Inputs", href: "inputs" },
    { title: "Inputs Graph", href: "inputs-graph" },
    { title: "Traffic Indicator", href: "traffic-indicator" },
    { title: "Flatmap", href: "flat-map" },
    { title: "Delta Bar", href: "delta-bar" },
    { title: "Trackmap", href: "track-map" },
    { title: "Twitch", href: "twitch-chat" },
  ] as const;

  return (
    <Sidebar>
      <SidebarHeader>
        <h1 className="text-primary pl-6 text-2xl">Race Vision</h1>
      </SidebarHeader>

      <SidebarContent className="px-4">
        <SidebarGroup>
          <SidebarGroupLabel>
            <h3 className="text-primary text-xs font-semibold tracking-wider uppercase">
              Overlay
            </h3>
          </SidebarGroupLabel>

          <SidebarGroupContent>
            <SidebarMenu>
              {routes.map(({ title, href }) => (
                <SidebarMenuItem key={title}>
                  <NavLink
                    to={href}
                    className={"font-medium tracking-tight capitalize"}
                  >
                    {({ isActive }) => (
                      <SidebarMenuButton asChild isActive={isActive}>
                        <span>{title}</span>
                      </SidebarMenuButton>
                    )}
                  </NavLink>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
