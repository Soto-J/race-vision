import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";

import { formatSessionTime } from "./lib/format";

import { TelemetryValueSchema } from "./lib/types";

import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [sessionTime, setSessionTime] = useState("");

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

  async function read_value() {
    try {
      const value = TelemetryValueSchema.parse(
        await invoke("read_value", {
          key: "SessionTime",
        })
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
    <main className="h-screen  w-screen">
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
    </main>
  );
}

export default App;
