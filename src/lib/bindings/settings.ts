import { invoke } from "@tauri-apps/api/core";

export interface RecentFile {
  path: string;
  title: string;
  position: number;
  timestamp: number;
}

export const getRecentFiles = () => invoke<RecentFile[]>("get_recent_files");
export const clearRecentFiles = () => invoke<void>("clear_recent_files");
