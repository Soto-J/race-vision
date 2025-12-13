import { Link } from "@tanstack/react-router";

import { cn } from "@/lib/utils";

import { HugeiconsIcon } from "@hugeicons/react";
import {
  ChartAverageIcon,
  DashboardSquare01Icon,
  EyeIcon,
  FuelStationIcon,
  JoystickIcon,
  MapsIcon,
  MapsLocation01Icon,
  MentoringIcon,
  TimerIcon,
  TrafficLightIcon,
  UserGroupIcon,
  BarChartIcon,
  TwitchIcon,
} from "@hugeicons/core-free-icons";

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
  useSidebar,
} from "@/routes/dashboard/components/sidebar";

export function AppSidebar() {
  const { open } = useSidebar();

  const routes = [
    {
      title: "General",
      href: "",
      icon: DashboardSquare01Icon,
    },
    {
      title: "Standings",
      href: "standings",
      icon: BarChartIcon,
    },
    {
      title: "Relative",
      href: "relative",
      icon: UserGroupIcon,
    },
    {
      title: "Fuel Calculator",
      href: "fuel-calculator",
      icon: FuelStationIcon,
    },
    {
      title: "Spotter",
      href: "spotter",
      icon: EyeIcon,
    },
    {
      title: "Pit Helper",
      href: "pit-helper",
      icon: MentoringIcon,
    },
    {
      title: "Inputs",
      href: "inputs",
      icon: JoystickIcon,
    },
    {
      title: "Inputs Graph",
      href: "inputs-graph",
      icon: ChartAverageIcon,
    },
    {
      title: "Traffic Indicator",
      href: "traffic-indicator",
      icon: TrafficLightIcon,
    },
    {
      title: "Flatmap",
      href: "flat-map",
      icon: MapsIcon,
    },
    {
      title: "Delta Bar",
      href: "delta-bar",
      icon: TimerIcon,
    },
    {
      title: "Trackmap",
      href: "track-map",
      icon: MapsLocation01Icon,
    },
    {
      title: "Twitch",
      href: "twitch-chat",
      icon: TwitchIcon,
    },
  ] as const;

  return (
    <Sidebar collapsible="icon">
      <SidebarHeader className="overflow-hidden">
        <h1
          className={cn(
            "text-primary pl-6 text-2xl whitespace-nowrap transition-all duration-300",
            open ? "opacity-100" : "hidden opacity-0",
          )}
        >
          Race Vision
        </h1>
      </SidebarHeader>

      <SidebarContent className={cn(open ? "px-4" : "")}>
        <SidebarGroup>
          <SidebarGroupLabel>
            <h3 className="text-primary text-xs font-semibold tracking-wider uppercase">
              Overlay
            </h3>
          </SidebarGroupLabel>

          <SidebarGroupContent>
            <SidebarMenu>
              {routes.map(({ title, href, icon }) => (
                <SidebarMenuItem key={title}>
                  <Link
                    to={`/dashboard/${href}`}
                    activeOptions={{ exact: true }}
                    className={"font-medium tracking-tight capitalize"}
                  >
                    {({ isActive }) => (
                      <SidebarMenuButton asChild isActive={isActive}>
                        <div>
                          <HugeiconsIcon icon={icon} />
                          <span>{title}</span>
                        </div>
                      </SidebarMenuButton>
                    )}
                  </Link>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
