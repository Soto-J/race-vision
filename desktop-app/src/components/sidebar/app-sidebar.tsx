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
    { title: "Fuel", href: "fuel" },
    { title: "Calculator", href: "calculator" },
    { title: "Spotter", href: "spotter" },
    { title: "Pit", href: "pit" },
    { title: "Helper", href: "helper" },
    { title: "Inputs", href: "inputs" },
    { title: "Inputs Graph", href: "inputs-graph" },
    { title: "Traffic", href: "traffic" },
    { title: "Indicator", href: "indicator" },
    { title: "Flatmap", href: "flat-map" },
    { title: "Delta", href: "delta" },
    { title: "Bar", href: "bar" },
    { title: "Trackmap", href: "track-map" },
    { title: "Twitch", href: "twitch" },
    { title: "Chat", href: "chat" },
  ] as const;

  return (
    <Sidebar>
      <SidebarHeader />
      <SidebarContent>
        <SidebarGroup />
        <SidebarGroupLabel className="mb-3">
          <h3 className="text-primary px-2 text-xs font-semibold tracking-wider uppercase">
            Overlay
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
