import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

import { useOverlayStore } from "./store/use-overlay-store";

export const useToggleEditMode = () => {
  const toggleEditMode = useOverlayStore((state) => state.toggleEditMode);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen("toggle-edit-mode", () => {
      toggleEditMode();
      console.log("F6 Clicked!");
    }).then((fn) => (unlisten = fn));

    return () => {
      unlisten?.();
    };
  }, []);
};
