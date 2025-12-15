import { useEffect, useRef } from "react";
import { useCanvasResize } from "@/hooks/use-canvas-resize";
import { useQueryClient } from "@tanstack/react-query";
import { TELEMETRY_QUERY_KEYS } from "@/lib/constants/telemetry-keys";
import {
  decodeTelemetryValue,
  TelemetrySnapshot,
  TelemetrySnapshotSchema,
} from "@/lib/types";
import { TelemetryVars } from "@/lib/constants/telemetry-vars";

const THROTTLE_COLOR = "#22c55e";
const BRAKE_COLOR = "#ef4444";
const CLUTCH_COLOR = "#2b35af";

const GRAPH_SPEED = 0.5;

const MOCK_DATA = () => {
  return { throttleValue: Math.random(), brakeValue: Math.random() };
};

const normalize = (value: number) => (value > 1 ? value / 100 : value);

interface PedalGraphProps {
  scrollSpeed?: number; // 0.5 = slower, 1 = normal, 2 = fast
  bufferSize?: number; // 300 = history depth
}

export const PedalGraph = ({
  scrollSpeed = 0.2,
  bufferSize = 300,
}: PedalGraphProps) => {
  const queryClient = useQueryClient();

  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  // Ring buffers
  const throttleRef = useRef(new Float32Array(bufferSize));
  const brakeRef = useRef(new Float32Array(bufferSize));
  const clutchRef = useRef(new Float32Array(bufferSize));

  // scrolling state
  const indexRef = useRef(0);
  const accumulatorRef = useRef(0);

  useCanvasResize(canvasRef);

  // ---------------------------------------------------
  // Recreate buffers ONLY when bufferSize changes
  // ---------------------------------------------------
  useEffect(() => {
    throttleRef.current = new Float32Array(bufferSize);
    brakeRef.current = new Float32Array(bufferSize);
    indexRef.current = 0;
    accumulatorRef.current = 0;
  }, [bufferSize]);

  // ---------------------------
  // Drawing animation loop
  // ---------------------------
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.lineJoin = "round";
    ctx.lineCap = "round";

    let frameId: number;

    const loop = () => {
      frameId = requestAnimationFrame(loop);

      // Scroll speed control
      accumulatorRef.current += scrollSpeed;
      while (accumulatorRef.current >= 1) {
        indexRef.current = (indexRef.current + 1) % bufferSize;
        accumulatorRef.current -= 1;
      }

      // const throttleValue = Math.random();
      // const brakeValue = Math.random();
      //TODO! query client
      let snapshot = queryClient.getQueryData<TelemetrySnapshot>(
        TELEMETRY_QUERY_KEYS.snapshot,
      );

      if (!snapshot) {
        return;
      }

      let throttleValue = decodeTelemetryValue(
        snapshot[TelemetryVars.THROTTLE],
      ) as number;

      // UPDATE BUFFERS
      throttleRef.current[indexRef.current] = throttleValue;

      // brakeRef.current[indexRef.current] = brakeValue;
      // clutchRef.current[indexRef.current] = clutchValue;

      // DRAWING
      const W = canvas.width;
      const H = canvas.height;
      const graphHeight = H;

      // Clear + redraw
      ctx.clearRect(0, 0, W, H);

      const drawLine = (buf: Float32Array, color: string) => {
        ctx.beginPath();

        for (let i = 0; i < buf.length; i++) {
          const idx = (indexRef.current + i) % buf.length;
          const x = (i / buf.length) * W;
          const y = graphHeight - buf[idx] * graphHeight;

          i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
        }

        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.stroke();
      };

      drawLine(throttleRef.current, THROTTLE_COLOR);
      // drawLine(brakeRef.current, BRAKE_COLOR);
      // drawLine(clutchRef.current, CLUTCH_COLOR);
    };
    loop();

    return () => cancelAnimationFrame(frameId);
  }, [scrollSpeed, bufferSize]);

  return (
    <canvas
      data-tauri-drag-region
      ref={canvasRef}
      width={1920}
      height={600}
      className="fixed inset-0 h-full w-full transform overflow-hidden"
    />
  );
};
