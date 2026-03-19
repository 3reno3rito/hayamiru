import { invoke } from "@tauri-apps/api/core";
import { setLocale } from "$lib/i18n/index.svelte";
import { setSubStyle } from "$lib/bindings/tracks";

export interface PlayerSettings {
  volume: number;
  speed: number;
  remember_position: boolean;
  auto_play: boolean;
  language: string;
  translate_lang: string;
  subtitle_style: SubtitleStyleSettings;
  os_api_key: string;
  os_username: string;
  os_password: string;
}

export interface SubtitleStyleSettings {
  font: string;
  size: number;
  color: string;
  border_color: string;
  border_size: number;
  position: number;
  bold: boolean;
}

const defaultSubStyle: SubtitleStyleSettings = {
  font: "Segoe UI", size: 55, color: "#ffffff",
  border_color: "#000000", border_size: 3, position: 100,
};

class SettingsStore {
  volume = $state(100);
  speed = $state(1.0);
  rememberPosition = $state(true);
  autoPlay = $state(true);
  language = $state("en");
  translateLang = $state("pt");
  osApiKey = $state("");
  osUsername = $state("");
  osPassword = $state("");

  // Subtitle style
  subFont = $state("Segoe UI");
  subSize = $state(55);
  subColor = $state("#ffffff");
  subBorderColor = $state("#000000");
  subBorderSize = $state(3);
  subPosition = $state(100);
  subBold = $state(false);

  #loaded = false;
  #saveTimer: ReturnType<typeof setTimeout> | null = null;

  async load() {
    try {
      const s: PlayerSettings = await invoke("get_settings");
      this.volume = s.volume;
      this.speed = s.speed;
      this.rememberPosition = s.remember_position;
      this.autoPlay = s.auto_play;
      this.language = s.language;
      this.translateLang = s.translate_lang ?? "pt";
      this.osApiKey = s.os_api_key ?? "";
      this.osUsername = s.os_username ?? "";
      this.osPassword = s.os_password ?? "";
      this.subFont = s.subtitle_style.font;
      this.subSize = s.subtitle_style.size;
      this.subColor = s.subtitle_style.color;
      this.subBorderColor = s.subtitle_style.border_color;
      this.subBorderSize = s.subtitle_style.border_size;
      this.subPosition = s.subtitle_style.position;
      this.subBold = s.subtitle_style.bold ?? false;
      setLocale(s.language);
      this.#loaded = true;
    } catch {}
  }

  save() {
    if (!this.#loaded) return;
    if (this.#saveTimer) clearTimeout(this.#saveTimer);
    this.#saveTimer = setTimeout(() => {
      invoke("save_settings", { newSettings: this.#toRust() }).catch(() => {});
    }, 300);
  }

  applySubStyle() {
    setSubStyle({
      font: this.subFont, size: this.subSize, color: this.subColor,
      border_color: this.subBorderColor, border_size: this.subBorderSize,
      position: this.subPosition, bold: this.subBold,
    });
    this.save();
  }

  resetSubStyle() {
    this.subFont = defaultSubStyle.font;
    this.subSize = defaultSubStyle.size;
    this.subColor = defaultSubStyle.color;
    this.subBorderColor = defaultSubStyle.border_color;
    this.subBorderSize = defaultSubStyle.border_size;
    this.subPosition = defaultSubStyle.position;
    this.subBold = false;
    this.applySubStyle();
  }

  resetAll() {
    this.volume = 100; this.speed = 1.0;
    this.rememberPosition = true; this.autoPlay = true;
    this.language = "en"; setLocale("en");
    this.resetSubStyle();
    this.save();
  }

  #toRust(): PlayerSettings {
    return {
      volume: this.volume, speed: this.speed,
      remember_position: this.rememberPosition, auto_play: this.autoPlay,
      language: this.language, translate_lang: this.translateLang,
      os_api_key: this.osApiKey, os_username: this.osUsername, os_password: this.osPassword,
      subtitle_style: {
        font: this.subFont, size: this.subSize, color: this.subColor,
        border_color: this.subBorderColor, border_size: this.subBorderSize,
        position: this.subPosition, bold: this.subBold,
      },
    };
  }
}

export const settings = new SettingsStore();
export const subFonts = ["Segoe UI", "Arial", "Tahoma", "Verdana", "Trebuchet MS", "Calibri", "Consolas", "Impact", "Georgia"];
