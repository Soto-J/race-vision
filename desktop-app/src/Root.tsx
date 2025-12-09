import { RouterProvider } from "react-router-dom";

import OverlayPage from "./overlay";
import { router } from "./router";

export default function Root() {
  const hash = window.location.hash;

  if (hash.includes("#/")) {
    const widgetId = hash.replace("#/", "");

    return <OverlayPage widgetId={widgetId} />;
  }

  return <RouterProvider router={router} />;
}
