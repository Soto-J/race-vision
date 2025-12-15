import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/widget/")({
  component: WidgetsPage,
});

export default function WidgetsPage() {
  return null;
}
