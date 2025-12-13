import { useRouter, createRootRoute } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ThemeProvider } from "@/components/theme-provider";

import DashboardLayout from "./dashboard/layout";
import WidgetLayout from "./widget/layout";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  const queryClient = new QueryClient();

  const router = useRouter();
  const isWidget = router.state.location.pathname.startsWith("/widget");

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        {isWidget ? <WidgetLayout /> : <DashboardLayout />}
      </ThemeProvider>
    </QueryClientProvider>
  );
}
