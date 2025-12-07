import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { formatSessionTime } from "@/lib/format";
import { VarKindSchema } from "@/lib/types";
import { TelemetryVars } from "@/lib/constants/telemetry-vars";

import "@/App.css";

function App() {
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
      const value = VarKindSchema.parse(
        await invoke("read_value", {
          key: TelemetryVars.SESSION_TIME,
        }),
      );

      console.log("Value: ", value);

      if ("F64" in value) {
        setSessionTime(String(value.F64[0]));
      }
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

export default App;
