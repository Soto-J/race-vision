export const formatFloat = (value: number, digits = 2): string =>
  value.toFixed(digits);

export const formatSessionTime = (secs: string | number): string => {
  const numSecs = typeof secs === "string" ? parseFloat(secs) : secs;

  if (typeof numSecs !== "number" || Number.isNaN(numSecs)) {
    return "--:--.---";
  }

  if (numSecs < 0) {
    return "--:--.---"; // iRacing sometimes uses -1
  }

  const hours = Math.floor(numSecs / 3600);
  const minutes = Math.floor((numSecs % 3600) / 60);
  const seconds = (numSecs % 60).toFixed(3).padStart(6, "0");

  if (hours > 0) {
    return `${hours}:${String(minutes).padStart(2, "0")}:${seconds}`;
  }

  return `${minutes}:${seconds}`;
};
