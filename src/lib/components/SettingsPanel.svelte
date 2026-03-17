<script lang="ts">
  import { setBrightness, setContrast, setSaturation, setVideoZoom, resetVideoZoomPan } from "$lib/bindings/video";
  import { setAudioNormalization, setAudioEqualizer, resetAudioEqualizer } from "$lib/bindings/audio-fx";

  let { visible = $bindable(false) }: { visible: boolean } = $props();
  let tab = $state<"general" | "video" | "audio" | "subtitles">("general");

  // General
  let volume = $state(100);
  let speed = $state(1.0);
  let rememberPosition = $state(true);
  let autoPlay = $state(true);

  // Video
  let brightness = $state(0);
  let contrast = $state(0);
  let saturation = $state(0);
  let zoom = $state(0);

  // Audio
  let normEnabled = $state(false);
  const eqLabels = ["60Hz", "230Hz", "910Hz", "3.6kHz", "14kHz"];
  const eqPresets: Record<string, number[]> = {
    "Flat": [0,0,0,0,0], "Bass": [8,5,0,0,0], "Treble": [0,0,0,4,8],
    "Vocal": [-2,0,4,4,0], "Rock": [4,2,-1,2,4],
  };
  let eqBands = $state([0,0,0,0,0]);

  // Subtitles
  let subFont = $state("Segoe UI");
  let subSize = $state(55);
  let subColor = $state("#FFFFFF");
  let subBorderColor = $state("#000000");
  let subBorderSize = $state(3);
  let subPosition = $state(100);

  const fonts = ["Segoe UI","Arial","Verdana","Tahoma","Calibri","Consolas","Times New Roman","Georgia","Noto Sans"];
  const speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];

  // Language
  let language = $state("en");
  const languages: Record<string, string> = {
    "en": "English",
    "pt": "Português",
    "es": "Español",
    "fr": "Français",
    "de": "Deutsch",
    "it": "Italiano",
    "ja": "日本語",
    "ko": "한국어",
    "zh": "中文",
    "ru": "Русский",
    "ar": "العربية",
    "hi": "हिन्दी",
    "tr": "Türkçe",
    "pl": "Polski",
    "nl": "Nederlands",
  };

  // Custom dropdown state
  let openDrop = $state<string | null>(null);
  function toggleDrop(id: string) { openDrop = openDrop === id ? null : id; }

  const tabs = [
    { id: "general" as const, label: "General" },
    { id: "video" as const, label: "Video" },
    { id: "audio" as const, label: "Audio" },
    { id: "subtitles" as const, label: "Subtitles" },
  ];

  function resetVideo() {
    brightness = 0; contrast = 0; saturation = 0; zoom = 0;
    setBrightness(0); setContrast(0); setSaturation(0); resetVideoZoomPan();
  }

  function applyEq() { setAudioEqualizer(eqBands).catch(() => {}); }
  function resetEq() { eqBands = [0,0,0,0,0]; resetAudioEqualizer(); }
  function setPreset(name: string) { eqBands = [...eqPresets[name]]; applyEq(); }

  function close() { visible = false; }
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[90] bg-black/40" onclick={close}></div>

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    data-panel
    class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-[91] w-[520px] h-[400px] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none overflow-hidden"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-white/[0.08]">
      <span class="font-medium text-sm">Settings</span>
      <button onclick={close} class="ctrl-btn w-7 h-7 text-xs">✕</button>
    </div>

    <div class="flex flex-1 min-h-0">
      <!-- Sidebar -->
      <div class="w-[120px] border-r border-white/[0.08] py-2 flex-shrink-0">
        {#each tabs as t}
          <button
            onclick={() => tab = t.id}
            class="w-full text-left px-4 py-2 text-[13px] transition-colors {tab === t.id ? 'bg-white/10 text-white' : 'text-white/50 hover:text-white/80 hover:bg-white/[0.05]'}"
          >{t.label}</button>
        {/each}
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        {#if tab === "general"}
          <div class="s-group">
            <div class="s-row">
              <span>Language</span>
              <div class="relative">
                <button class="s-drop-btn" onclick={() => toggleDrop("lang")}>{languages[language]} <span class="opacity-40">▾</span></button>
                {#if openDrop === "lang"}
                  <div class="s-drop-list">
                    {#each Object.entries(languages) as [code, name]}
                      <button class="s-drop-item {language === code ? 'text-blue-400' : ''}" onclick={() => { language = code; openDrop = null; }}>{name}</button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          </div>
          <div class="s-group">
            <label class="s-row">
              <span>Default Volume</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="100" bind:value={volume} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right">{volume}%</span>
              </div>
            </label>
            <div class="s-row">
              <span>Default Speed</span>
              <div class="relative">
                <button class="s-drop-btn" onclick={() => toggleDrop("speed")}>{speed}x <span class="opacity-40">▾</span></button>
                {#if openDrop === "speed"}
                  <div class="s-drop-list">
                    {#each speeds as s}
                      <button class="s-drop-item {speed === s ? 'text-blue-400' : ''}" onclick={() => { speed = s; openDrop = null; }}>{s}x</button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          </div>
          <div class="s-group">
            <label class="s-toggle"><span>Remember Position</span><input type="checkbox" bind:checked={rememberPosition} /></label>
            <label class="s-toggle"><span>Auto Play</span><input type="checkbox" bind:checked={autoPlay} /></label>
          </div>

        {:else if tab === "video"}
          <div class="s-group">
            <div class="flex items-center justify-between mb-1">
              <span class="text-white/50 text-xs uppercase tracking-wide">Color</span>
              <button onclick={resetVideo} class="text-xs text-white/40 hover:text-white/70">Reset</button>
            </div>
            {#each [
              { label: "Brightness", get: () => brightness, set: (v: number) => { brightness = v; setBrightness(v); } },
              { label: "Contrast", get: () => contrast, set: (v: number) => { contrast = v; setContrast(v); } },
              { label: "Saturation", get: () => saturation, set: (v: number) => { saturation = v; setSaturation(v); } },
            ] as ctrl}
              <label class="s-row">
                <span>{ctrl.label}</span>
                <div class="flex items-center gap-2">
                  <input type="range" min="-100" max="100" value={ctrl.get()} oninput={(e) => ctrl.set(Number((e.target as HTMLInputElement).value))} class="s-range flex-1" />
                  <span class="text-white/50 w-8 text-right">{ctrl.get()}</span>
                </div>
              </label>
            {/each}
            <label class="s-row">
              <span>Zoom</span>
              <div class="flex items-center gap-2">
                <input type="range" min="-1" max="3" step="0.1" bind:value={zoom} oninput={() => setVideoZoom(zoom)} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right">{zoom.toFixed(1)}</span>
              </div>
            </label>
          </div>

        {:else if tab === "audio"}
          <div class="s-group">
            <label class="s-toggle"><span>Normalization</span>
              <input type="checkbox" bind:checked={normEnabled} onchange={() => setAudioNormalization(normEnabled)} />
            </label>
          </div>
          <div class="s-group">
            <div class="flex items-center justify-between mb-1">
              <span class="text-white/50 text-xs uppercase tracking-wide">Equalizer</span>
              <button onclick={resetEq} class="text-xs text-white/40 hover:text-white/70">Reset</button>
            </div>
            <div class="flex gap-1 flex-wrap mb-2">
              {#each Object.keys(eqPresets) as name}
                <button class="px-2 py-0.5 rounded text-[11px] bg-white/[0.08] hover:bg-white/[0.15] text-white/70" onclick={() => setPreset(name)}>{name}</button>
              {/each}
            </div>
            {#each eqLabels as label, i}
              <div class="flex items-center gap-2">
                <span class="w-12 text-[11px] text-white/50 text-right">{label}</span>
                <input type="range" min="-12" max="12" step="1" bind:value={eqBands[i]} oninput={applyEq} class="s-range flex-1" />
                <span class="w-6 text-[11px] text-white/50 text-right">{eqBands[i] > 0 ? "+" : ""}{eqBands[i]}</span>
              </div>
            {/each}
          </div>

        {:else if tab === "subtitles"}
          <div class="s-group">
            <div class="s-row">
              <span>Font</span>
              <div class="relative">
                <button class="s-drop-btn" onclick={() => toggleDrop("font")}>{subFont} <span class="opacity-40">▾</span></button>
                {#if openDrop === "font"}
                  <div class="s-drop-list">
                    {#each fonts as f}
                      <button class="s-drop-item {subFont === f ? 'text-blue-400' : ''}" style="font-family:'{f}'" onclick={() => { subFont = f; openDrop = null; }}>{f}</button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
            <label class="s-row">
              <span>Size</span>
              <div class="flex items-center gap-2">
                <input type="range" min="10" max="100" bind:value={subSize} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right">{subSize}</span>
              </div>
            </label>
            <div class="flex items-center justify-between py-1">
              <span>Text Color</span>
              <input type="color" bind:value={subColor} class="s-color" />
            </div>
            <div class="flex items-center justify-between py-1">
              <span>Border Color</span>
              <input type="color" bind:value={subBorderColor} class="s-color" />
            </div>
            <label class="s-row">
              <span>Border Size</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="5" step="0.5" bind:value={subBorderSize} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right">{subBorderSize}</span>
              </div>
            </label>
            <label class="s-row">
              <span>Position</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="100" bind:value={subPosition} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right">{subPosition}%</span>
              </div>
            </label>
          </div>
        {/if}
      </div>
    </div>

    <!-- Footer -->
    <div class="flex items-center justify-end px-4 py-2 border-t border-white/[0.08]">
      <span class="text-[11px] text-white/20">Hayamiru v0.1.0</span>
    </div>
  </div>
{/if}

<style>
  .s-group { display: flex; flex-direction: column; gap: 8px; padding-bottom: 12px; border-bottom: 1px solid rgba(255,255,255,0.06); }
  .s-group:last-child { border-bottom: none; padding-bottom: 0; }
  .s-row { display: flex; flex-direction: column; gap: 4px; color: rgba(255,255,255,0.8); cursor: default; }
  .s-toggle { display: flex; align-items: center; justify-content: space-between; color: rgba(255,255,255,0.8); cursor: pointer; padding: 4px 0; }
  .s-toggle input[type="checkbox"] {
    appearance: none; width: 36px; height: 20px; background: rgba(255,255,255,0.15);
    border-radius: 10px; position: relative; cursor: pointer; transition: background 0.2s; flex-shrink: 0;
  }
  .s-toggle input[type="checkbox"]::after {
    content: ""; position: absolute; top: 2px; left: 2px; width: 16px; height: 16px;
    background: white; border-radius: 50%; transition: transform 0.2s;
  }
  .s-toggle input[type="checkbox"]:checked { background: oklch(0.65 0.25 250); }
  .s-toggle input[type="checkbox"]:checked::after { transform: translateX(16px); }
  .s-range {
    appearance: none; height: 4px; background: rgba(255,255,255,0.15);
    border-radius: 2px; outline: none; cursor: pointer;
  }
  .s-range::-webkit-slider-thumb {
    appearance: none; width: 14px; height: 14px; background: oklch(0.65 0.25 250);
    border-radius: 50%; cursor: pointer;
  }
  .s-drop-btn {
    width: 100%; display: flex; align-items: center; justify-content: space-between;
    background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.1);
    border-radius: 6px; color: rgba(255,255,255,0.9); padding: 6px 10px;
    font-size: 13px; cursor: pointer;
  }
  .s-drop-btn:hover { background: rgba(255,255,255,0.12); }
  .s-drop-list {
    position: absolute; left: 0; right: 0; top: 100%; margin-top: 4px; z-index: 10;
    max-height: 200px; overflow-y: auto;
    background: #1a1a1f; border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.5); padding: 4px 0;
  }
  .s-drop-item {
    width: 100%; text-align: left; padding: 6px 12px; font-size: 13px;
    color: rgba(255,255,255,0.8); background: none; border: none; cursor: pointer;
  }
  .s-drop-item:hover { background: rgba(255,255,255,0.1); }
  .s-color {
    appearance: none; width: 32px; height: 24px; border: 1px solid rgba(255,255,255,0.15);
    border-radius: 4px; cursor: pointer; padding: 0; background: transparent;
  }
  .s-color::-webkit-color-swatch-wrapper { padding: 2px; }
  .s-color::-webkit-color-swatch { border-radius: 2px; border: none; }
</style>
