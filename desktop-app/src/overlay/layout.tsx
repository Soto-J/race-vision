import React from "react";

interface OverlayPageLayoutProps {
  children: React.ReactNode;
}
export default function OverlayPageLayout({
  children,
}: OverlayPageLayoutProps) {
  return <main className="text-9xl bg-transparent">{children}</main>;
}
