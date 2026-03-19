import { invoke } from "@tauri-apps/api/core";

export interface SubResult {
  name: string;
  lang: string;
  download_count: number;
  rating: number;
  file_id: number;
}

export const searchSubtitles = (lang: string) => invoke<SubResult[]>("search_subtitles", { lang });
export const downloadSubtitle = (fileId: number, fileName: string) => invoke<string>("download_subtitle", { fileId, fileName });
