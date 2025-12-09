import { createHashRouter } from "react-router-dom";
import AppLayout from "./app-layout";

import App from ".";
import Standings from "./pages/standings";
import Relative from "./pages/relative";
import FuelCalculator from "./pages/fuel-calculator";
import Spotter from "./pages/spotter";
import PitHelper from "./pages/pit-helper";
import Inputs from "./pages/inputs";
import InputsGraph from "./pages/inputs-graph";
import TrafficIndicator from "./pages/traffic-indicator";
import Flatmap from "./pages/flat-map";
import TrackMap from "./pages/track-map";
import DeltaBar from "./pages/delta-bar";
import TwitchChat from "./pages/twitch";

export const router = createHashRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <App /> },

      { path: "standings", element: <Standings /> },
      { path: "relative", element: <Relative /> },
      { path: "fuel-calculator", element: <FuelCalculator /> },
      { path: "spotter", element: <Spotter /> },
      { path: "pit-helper", element: <PitHelper /> },
      { path: "inputs", element: <Inputs /> },
      { path: "inputs-graph", element: <InputsGraph /> },
      { path: "traffic-indicator", element: <TrafficIndicator /> },
      { path: "flat-map", element: <Flatmap /> },
      { path: "delta-bar", element: <DeltaBar /> },
      { path: "track-map", element: <TrackMap /> },
      { path: "twitch-chat", element: <TwitchChat /> },
    ],
  },
]);
