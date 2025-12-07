import React from "react";
import ReactDOM from "react-dom/client";

import OverlayPage from "./overlay";


const overlayRoot = document.getElementById("overlay-root") as HTMLElement;

if (!overlayRoot) {
  throw new Error("Overlay root element #overlay-root not found");
}

ReactDOM.createRoot(overlayRoot).render(
  <React.StrictMode>
    <OverlayPage />
  </React.StrictMode>
);
