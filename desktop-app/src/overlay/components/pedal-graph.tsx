import { useEffect, useRef } from "react";

export function PedalGraph() {
  const getPedalValue = () => {};
  const canvasRef = useRef(null);
  const dataRef = useRef(new Float32Array(300)); // 300-sample buffer
  const indexRef = useRef(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    function loop() {
      requestAnimationFrame(loop);

      // Get latest pedal value (0–1)
      const v = getPedalValue();

      // Write into circular buffer
      dataRef.current[indexRef.current] = v;
      indexRef.current = (indexRef.current + 1) % dataRef.current.length;

      // Clear canvas
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Draw graph
      ctx.beginPath();
      for (let i = 0; i < dataRef.current.length; i++) {
        const idx = (indexRef.current + i) % dataRef.current.length;
        const x = (i / dataRef.current.length) * canvas.width;
        const y = canvas.height - dataRef.current[idx] * canvas.height;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.strokeStyle = "#22c55e"; // green
      ctx.lineWidth = 2;
      ctx.stroke();
    }

    loop();
  }, []);

  return <canvas ref={canvasRef} width={300} height={80} />;
}
