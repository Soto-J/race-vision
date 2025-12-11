interface RelativeProps {
  scrollSpeed?: number; // 0.5 = slower, 1 = normal, 2 = fast
  bufferSize?: number; // 300 = history depth
}

export const Relative = () => {
  return (
    <div data-tauri-drag-region className="h-screen">
      Relative
    </div>
  );
};
