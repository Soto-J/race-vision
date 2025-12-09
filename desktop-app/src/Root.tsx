import { RouterProvider } from "react-router-dom";

import OverlayPage from "./overlay";
import { router } from "./router";

export default function Root() {
  const hash = window.location.hash;

  if (hash.includes("#/overlay/")) {
    const widgetId = hash.replace("#/overlay/", "");

    return <OverlayPage widgetId={widgetId} />;
  }

  return <RouterProvider router={router} />;
}
