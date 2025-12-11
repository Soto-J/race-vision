import { useEffect } from "react";

import { listen } from "@tauri-apps/api/event";

import { useOverlayStore } from "../store/use-overlay-store";

export const useEditModeListener = () => {
  const toggleEditMode = useOverlayStore((state) => state.toggleEditMode);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listen<boolean>("toggle-edit-mode", (event) => {
      toggleEditMode(event.payload);
    }).then((fn) => (unlisten = fn));

    return () => {
      unlisten?.();
    };
  }, []);
};
