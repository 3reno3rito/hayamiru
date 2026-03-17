import { invoke } from "@tauri-apps/api/core";

export const initPlayer = () => invoke<void>("init_player");
export const openFile = (path: string) => invoke<void>("open_file", { path });
export const togglePause = () => invoke<void>("toggle_pause");
export const play = () => invoke<void>("play");
export const pause = () => invoke<void>("pause");
export const stop = () => invoke<void>("stop");
export const seekRelative = (seconds: number) => invoke<void>("seek_relative", { seconds });
export const seekAbsolute = (seconds: number) => invoke<void>("seek_absolute", { seconds });
export const setVolume = (volume: number) => invoke<void>("set_volume", { volume });
export const setSpeed = (speed: number) => invoke<void>("set_speed", { speed });

export interface PlaybackState {
  time_pos: number;
  duration: number;
  paused: boolean;
  title: string;
  volume: number;
}

export const getPlaybackState = () => invoke<PlaybackState>("get_playback_state");
