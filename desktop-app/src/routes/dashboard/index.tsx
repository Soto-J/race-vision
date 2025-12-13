import { useEffect, useState } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";
import { formatSessionTime } from "@/lib/format";
import { TelemetryVars } from "@/lib/constants/telemetry-vars";
import { TelemetryValueSchema } from "@/lib/types";

export const Route = createFileRoute("/dashboard/")({
  component: () => DashboardHome,
});

function DashboardHome() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [sessionTime, setSessionTime] = useState("");
  const [selectedVars, setSelectedVars] = useState<string[]>([]);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        await read_value();
      } catch (error) {
        console.error("Error reading value:", error);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const watchedVars = async () => {
    try {
      await invoke("set_watched_vars", { vars: selectedVars });
    } catch (error) {
      console.error(error);
    }
  };

  async function read_value() {
    try {
      const value = TelemetryValueSchema.safeParse(
        await invoke("read_value", {
          key: TelemetryVars.SESSION_TIME,
        }),
      );

      if (!value.success) {
        console.error("Error", value.error);
        return;
      }

      setSessionTime(String(value.data.value));
    } catch (error) {
      console.error("Failed to read session time:", error);
    }
  }

  return (
    <div>
      <h1>Session Time: {formatSessionTime(sessionTime)}</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </div>
  );
}
