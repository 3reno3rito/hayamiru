import { invoke } from "@tauri-apps/api/core";

export async function toggleFullscreen(): Promise<void> {
  document.documentElement.classList.add("fs-transition");
  await invoke("toggle_fullscreen");
  setTimeout(() => {
    document.documentElement.classList.remove("fs-transition");
  }, 200);
}

export const setAlwaysOnTop = (enabled: boolean) => invoke<void>("set_always_on_top", { enabled });
export const minimizeWindow = () => invoke<void>("minimize_window");
export const maximizeWindow = () => invoke<void>("maximize_window");
export const closeWindow = () => invoke<void>("close_window");
