import { RefObject, useEffect } from "react";

export function useCanvasResize(
  canvasRef: RefObject<HTMLCanvasElement | null>,
) {
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    let resizeQueued = false;

    const applyResize = () => {
      resizeQueued = false;

      const parent = canvas.parentElement;
      if (!parent) return;

      const width = parent.clientWidth;
      const height = parent.clientHeight;

      // Resize pixel buffer (expensive)
      canvas.width = width;
      canvas.height = height;

      // 💥 IMPORTANT: Immediately paint a background to avoid flicker
      const ctx = canvas.getContext("2d");
      if (ctx) {
        ctx.fillStyle = "rgba(0,0,0,0)"; // transparent
        ctx.fillRect(0, 0, width, height);
      }
    };

    const queueResize = () => {
      if (!resizeQueued) {
        resizeQueued = true;
        requestAnimationFrame(applyResize);
      }
    };

    const observer = new ResizeObserver(queueResize);
    observer.observe(canvas.parentElement!);

    // Initial fire
    queueResize();

    return () => observer.disconnect();
  }, [canvasRef]);
}
