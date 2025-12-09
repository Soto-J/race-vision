import { useEffect, useRef } from "react";

import { useTelemetryStore } from "@/hooks/store/use-telemetry-store";

import { TelemetryVars } from "@/lib/constants/telemetry-vars";

const THROTTLE_COLOR = "#22c55e";
const BRAKE_COLOR = "#ef4444";

const GRAPH_SPEED = 0.5;

interface RelativeProps {
  scrollSpeed?: number; // 0.5 = slower, 1 = normal, 2 = fast
  bufferSize?: number; // 300 = history depth
}

export const Relative = ({
  scrollSpeed = 1,
  bufferSize = 300,
}: RelativeProps) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  const throttleRef = useRef(new Float32Array(bufferSize));
  const brakeRef = useRef(new Float32Array(bufferSize));
  const clutchRef = useRef(new Float32Array(bufferSize));

  const indexRef = useRef(0);
  const accumulatorRef = useRef(0); // for fractional scrollSpeed logic

  const normalize = (v: number) => (v > 1 ? v / 100 : v);

  useEffect(() => {
    // Recreate buffers when size changes
    throttleRef.current = new Float32Array(bufferSize);
    brakeRef.current = new Float32Array(bufferSize);
    indexRef.current = 0;
    accumulatorRef.current = 0;

    const canvas = canvasRef.current;

    if (!canvas) return;

    const ctx = canvas.getContext("2d");

    if (!ctx) return;

    ctx.lineJoin = "round";
    ctx.lineCap = "round";

    let frameId: number;

    const loop = () => {
      frameId = requestAnimationFrame(loop);

      // pull LIVE telemetry values
      const store = useTelemetryStore.getState();

      const throttleValue = normalize(
        (store.getValue(TelemetryVars.THROTTLE) as number) ?? 0,
      );
      const brakeValue = normalize(
        (store.getValue(TelemetryVars.BRAKE) as number) ?? 0,
      );

      // ------------------------------------
      // SCROLL SPEED CONTROL
      // ------------------------------------
      accumulatorRef.current += scrollSpeed;

      while (accumulatorRef.current >= 1) {
        indexRef.current = (indexRef.current + 1) % bufferSize;
        accumulatorRef.current -= 1;
      }

      // Update buffers
      throttleRef.current[indexRef.current] = throttleValue;
      brakeRef.current[indexRef.current] = brakeValue;
      // clutchRef.current[indexRef.current] = pedalValues;

      // ------------------------------------
      // DRAWING
      // ------------------------------------
      const W = canvas.width;
      const H = canvas.height;
      const graphHeight = H * 0.7;

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
      drawLine(brakeRef.current, BRAKE_COLOR);
    };

    loop();

    return () => {
      cancelAnimationFrame(frameId);
    };
  }, [scrollSpeed, bufferSize]);

  return <canvas ref={canvasRef} width={300} height={80} />;
};
