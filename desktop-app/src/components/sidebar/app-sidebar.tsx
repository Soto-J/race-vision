import { NavLink } from "react-router-dom";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/sidebar";

export function AppSidebar() {
  const items = [
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
      <SidebarHeader />
      <SidebarContent>
        <SidebarGroup />
        <SidebarGroupLabel className="mb-3">
          <h3 className="text-primary px-2 text-xs font-semibold tracking-wider uppercase">
            Race Vision
          </h3>
        </SidebarGroupLabel>

        <SidebarGroupContent>
          <SidebarMenu>
            {items.map(({ title, href }) => (
              <SidebarMenuItem key={title}>
                <SidebarMenuButton asChild>
                  <NavLink
                    to={href}
                    className="font-medium tracking-tight capitalize"
                  >
                    {title}
                  </NavLink>
                </SidebarMenuButton>
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroupContent>
        <SidebarGroup />
      </SidebarContent>
      <SidebarFooter />
    </Sidebar>
  );
}
