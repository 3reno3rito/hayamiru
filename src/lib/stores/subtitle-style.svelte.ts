import { setSubStyle } from "$lib/bindings/tracks";
import { getSettings, saveSettings, type PlayerSettings } from "$lib/bindings/settings";

class SubtitleStyleStore {
  font = $state("Segoe UI");
  size = $state(55);
  color = $state("#ffffff");
  borderColor = $state("#000000");
  borderSize = $state(3);
  position = $state(100);

  #timer: ReturnType<typeof setTimeout> | null = null;
  #settings: PlayerSettings | null = null;

  loadFrom(settings: PlayerSettings) {
    this.#settings = settings;
    const s = settings.subtitle_style;
    this.font = s.font;
    this.size = s.size;
    this.color = s.color;
    this.borderColor = s.border_color;
    this.borderSize = s.border_size;
    this.position = s.position;
  }

  apply() {
    if (this.#timer) clearTimeout(this.#timer);
    this.#timer = setTimeout(() => {
      setSubStyle({
        font: this.font,
        size: this.size,
        color: this.color,
        border_color: this.borderColor,
        border_size: this.borderSize,
        position: this.position,
      });
      this.#save();
    }, 150);
  }

  reset() {
    this.font = "Segoe UI";
    this.size = 55;
    this.color = "#ffffff";
    this.borderColor = "#000000";
    this.borderSize = 3;
    this.position = 100;
    this.apply();
  }

  async #save() {
    try {
      if (!this.#settings) this.#settings = await getSettings();
      this.#settings.subtitle_style = {
        font: this.font,
        size: this.size,
        color: this.color,
        border_color: this.borderColor,
        border_size: this.borderSize,
        position: this.position,
      };
      await saveSettings(this.#settings);
    } catch {}
  }
}

export const subStyle = new SubtitleStyleStore();
export const subFonts = ["Segoe UI", "Arial", "Tahoma", "Verdana", "Trebuchet MS", "Calibri", "Consolas", "Impact", "Georgia"];
