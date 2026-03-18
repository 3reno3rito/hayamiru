import { invoke } from "@tauri-apps/api/core";

export interface RecentFile {
  path: string;
  title: string;
  position: number;
  timestamp: number;
}

export interface PlayerSettings {
  volume: number;
  speed: number;
  remember_position: boolean;
  auto_play: boolean;
  language: string;
  recent_files: RecentFile[];
  subtitle_style: SubtitleStyleSettings;
}

export interface SubtitleStyleSettings {
  font: string;
  size: number;
  color: string;
  border_color: string;
  border_size: number;
  position: number;
}

export const getSettings = () => invoke<PlayerSettings>("get_settings");
export const saveSettings = (settings: PlayerSettings) => invoke<void>("save_settings", { newSettings: settings });
export const getRecentFiles = () => invoke<RecentFile[]>("get_recent_files");
export const clearRecentFiles = () => invoke<void>("clear_recent_files");
