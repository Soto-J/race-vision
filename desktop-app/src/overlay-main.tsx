import React from "react";
import ReactDOM from "react-dom/client";

import OverlayPage from "./overlay";

ReactDOM.createRoot(
  document.getElementById("root-overlay") as HTMLElement
).render(
  <React.StrictMode>
    <OverlayPage />
  </React.StrictMode>
);
