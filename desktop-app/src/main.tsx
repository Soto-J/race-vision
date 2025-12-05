import React from "react";
import ReactDOM from "react-dom/client";

import { createHashRouter, RouterProvider } from "react-router-dom";

import App from "./pages/app";

import Standings from "./pages/standings";
import Relative from "./pages/relative";
import Fuel from "./pages/fuel";
import Calculator from "./pages/calculator";
import Spotter from "./pages/spotter";
import Pit from "./pages/pit";
import Helper from "./pages/helper";
import Inputs from "./pages/inputs";
import InputsGraph from "./pages/inputs-graph";
import Traffic from "./pages/traffic";
import Indicator from "./pages/indicator";
import Flatmap from "./pages/flat-map";
import Delta from "./pages/delta";
import Bar from "./pages/bar";
import TrackMap from "./pages/track-map";
import Twitch from "./pages/twitch";
import Chat from "./pages/chat";
import AppLayout from "./app-layout";

const router = createHashRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <App /> },

      { path: "standings", element: <Standings /> },
      { path: "relative", element: <Relative /> },
      { path: "fuel", element: <Fuel /> },
      { path: "calculator", element: <Calculator /> },
      { path: "spotter", element: <Spotter /> },
      { path: "pit", element: <Pit /> },
      { path: "helper", element: <Helper /> },
      { path: "inputs", element: <Inputs /> },
      { path: "inputs-graph", element: <InputsGraph /> },
      { path: "traffic", element: <Traffic /> },
      { path: "indicator", element: <Indicator /> },
      { path: "flat-map", element: <Flatmap /> },
      { path: "delta", element: <Delta /> },
      { path: "bar", element: <Bar /> },
      { path: "track-map", element: <TrackMap /> },
      { path: "twitch", element: <Twitch /> },
      { path: "chat", element: <Chat /> },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
