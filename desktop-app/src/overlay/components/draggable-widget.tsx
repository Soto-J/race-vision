import { useEffect, useState } from "react";

import { Rnd } from "react-rnd";
import { useOverlayStore } from "../../hooks/store/use-overlay-store";

const SNAP = 12;

const maybeSnap = (a: number, b: number) => {
  return Math.abs(a - b) < SNAP ? b : a;
};

interface DraggableOverlayProps {
  children: React.ReactNode;
  id: string;
  width: number;
  height: number;
}

export const DraggableWidget = ({
  children,
  id,
  width,
  height,
}: DraggableOverlayProps) => {
  const [pos, setPos] = useState({ x: 200, y: 200 });

  const widgets = useOverlayStore((s) => s.widgets);
  const registerWidget = useOverlayStore((s) => s.registerWidget);
  const updateWidget = useOverlayStore((s) => s.updateWidget);

  // Register/Update widget dimensions & initial position
  useEffect(() => {
    registerWidget(id, { x: pos.x, y: pos.y, width, height });
  }, [id]);

  return (
    <Rnd
      className="overlay-widget"
      bounds="window"
      position={pos}
      size={{ width, height }}
      dragHandleClassName="drag-handle"
      onDragStop={(_, { x, y }) => {
        // Snap to screen edges
        x = maybeSnap(x, 0);
        y = maybeSnap(y, 0);

        x = maybeSnap(x, window.innerWidth - width);
        y = maybeSnap(y, window.innerHeight - height);

        // Snap to other widgets
        Object.entries(widgets).forEach(([otherId, w]) => {
          if (otherId === id) return;

          // snap left edges
          x = maybeSnap(x, w.x);

          // snap top edges
          y = maybeSnap(y, w.y);

          // snap right edges
          x = maybeSnap(x + width, w.x + w.width) - width;

          // snap bottom edges
          y = maybeSnap(y + height, w.y + w.height) - height;

          // vertical alignment line snapping
          x = maybeSnap(x, w.x + w.width / 2 - width / 2);

          // horizontal alignment snapping
          y = maybeSnap(y, w.y + w.height / 2 - height / 2);
        });

        setPos({ x, y });
        updateWidget(id, { x, y });
      }}
    >
      <div className="drag-handle cursor-move bg-neutral-800 p-1 text-xs">
        Drag
      </div>
      {children}
    </Rnd>
  );
};
