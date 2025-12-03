import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";

import OverlayPage from "./overlay";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider
      router={createBrowserRouter([
        {
          id: "root",
          path: "/",
          element: <App />,
        },
        {
          path: "/overlay",
          element: <OverlayPage />,
        },
      ])}
    />
  </React.StrictMode>
);
