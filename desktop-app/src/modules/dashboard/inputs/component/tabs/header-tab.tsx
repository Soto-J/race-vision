import { DisplayOptions } from "@/modules/dashboard/components/display-options";
import type { FeatureKey } from "@/hooks/settings/helper";
import type { InputsSettings } from "../../types";

interface HeaderTabProps {
  settings: InputsSettings["header"];
  toggleFeature: (feature: FeatureKey<"header">) => void;
}

export const HeaderTab = ({ settings, toggleFeature }: HeaderTabProps) => {
  const options = [
    { title: "Session name", key: "sessionName" as const },
    { title: "Event type", key: "eventType" as const },
    { title: "Track name", key: "trackName" as const },
    { title: "Local time (24h)", key: "localtime24h" as const },
    { title: "Local time (am/pm)", key: "localtimeAmPm" as const },
    { title: "In sim time (24h)", key: "inSimTime24h" as const },
    { title: "In sim time (am/pm)", key: "inSimTimeAmPm" as const },
    { title: "Air temp", key: "airTemp" as const },
    { title: "Track temp", key: "trackTemp" as const },
    { title: "Humidity", key: "humidity" as const },
    { title: "Fog level", key: "fogLevel" as const },
    { title: "Time remaining", key: "timeRemaining" as const },
    { title: "Laps remaining", key: "lapsRemaining" as const },
    { title: "Incident count", key: "incidentCount" as const },
    { title: "Current lap time", key: "currentLapTime" as const },
    { title: "Session best lap time", key: "sessionBestLapTime" as const },
    { title: "Last lap time", key: "lastLapTime" as const },
    { title: "Last lap time calculated", key: "lastLapTimeCalculated" as const },
    { title: "Lap delta best", key: "lapDeltaBest" as const },
    { title: "Lap delta optimal", key: "lapDeltaOptimal" as const },
    { title: "Lap delta session best", key: "lapDeltaSessionBest" as const },
    { title: "Lap delta session optimal", key: "lapDeltaSessionOptimal" as const },
    { title: "Lap delta session last", key: "lapDeltaSessionLast" as const },
    { title: "Brake bias", key: "brakeBias" as const },
    { title: "Fuel level", key: "fuelLevel" as const },
    { title: "Water temp", key: "waterTemp" as const },
    { title: "Oil temp", key: "oilTemp" as const },
    { title: "SOF", key: "sof" as const },
    { title: "Current stint in laps", key: "currentStintInLaps" as const },
    { title: "Current stint in time", key: "currentStintInTime" as const },
    { title: "RPM", key: "rpm" as const },
    { title: "Deploy mode", key: "deployMode" as const },
    { title: "ARB front", key: "arbFront" as const },
    { title: "ARB rear", key: "arbRear" as const },
    { title: "ABS", key: "abs" as const },
    { title: "TC1", key: "tc1" as const },
    { title: "TC2", key: "tc2" as const },
    { title: "Weight jacker", key: "weightJacker" as const },
    { title: "Rear brake valve", key: "rearBrakeValve" as const },
    { title: "Precipitation", key: "precipitation" as const },
    { title: "Track wetness", key: "trackWetness" as const },
    { title: "Weather declared wet", key: "weatherDeclaredWet" as const },
    { title: "Pit time loss beta", key: "pitTimeLossBeta" as const },
    { title: "Wind direction for driver", key: "windDirectionForDriver" as const },
    { title: "Predicted position after pit stop", key: "predictedPositionAfterPitStop" as const },
    { title: "iRating and gain", key: "iRatingAndGain" as const },
    { title: "Push to pass", key: "pushToPass" as const },
  ];

  return <DisplayOptions settings={settings} options={options} toggleFeature={toggleFeature} />;
};
