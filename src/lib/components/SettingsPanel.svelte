<script lang="ts">
  import { setBrightness, setContrast, setSaturation, setVideoZoom, resetVideoZoomPan, toggleDeinterlace } from "$lib/bindings/video";
  import { setAudioNormalization, setAudioEqualizer, resetAudioEqualizer } from "$lib/bindings/audio-fx";
  import { settings, subFonts } from "$lib/stores/settings.svelte";
  import { keybindings } from "$lib/stores/keybindings.svelte";
  import { t, setLocale } from "$lib/i18n/index.svelte";
  import Select from "./Select.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();
  let tab = $state<"general" | "video" | "audio" | "subtitles" | "shortcuts">("general");
  let rebinding = $state<string | null>(null);

  function handleRebind(e: KeyboardEvent) {
    if (!rebinding) return;
    e.preventDefault(); e.stopPropagation();
    if (e.key === "Escape") { rebinding = null; return; }
    const parts: string[] = [];
    if (e.ctrlKey) parts.push("Ctrl");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");
    if (!["Control", "Shift", "Alt"].includes(e.key)) parts.push(e.key === " " ? "Space" : e.key);
    if (parts.length === 0) return;
    keybindings.setKey(rebinding, parts.join("+"));
    rebinding = null;
    settings.save();
  }

  // Video (local — not persisted, applied directly to mpv)
  let brightness = $state(0);
  let contrast = $state(0);
  let saturation = $state(0);
  let zoom = $state(0);
  let deinterlace = $state(false);

  // Audio (local)
  let normEnabled = $state(false);
  const eqLabels = ["60Hz", "230Hz", "910Hz", "3.6kHz", "14kHz"];
  const eqPresets: Record<string, number[]> = {
    "Flat": [0,0,0,0,0], "Bass": [8,5,0,0,0], "Treble": [0,0,0,4,8],
    "Vocal": [-2,0,4,4,0], "Rock": [4,2,-1,2,4],
  };
  let eqBands = $state([0,0,0,0,0]);
  const speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];

  const languages: Record<string, string> = {
    "en": "English", "pt": "Português", "es": "Español", "fr": "Français",
    "de": "Deutsch", "it": "Italiano", "ja": "日本語", "ko": "한국어",
    "zh": "中文", "ru": "Русский", "ar": "العربية", "hi": "हिन्दी",
  };

  const tabs = [
    { id: "general" as const, get label() { return t().general; } },
    { id: "video" as const, get label() { return t().video; } },
    { id: "audio" as const, get label() { return t().audio; } },
    { id: "subtitles" as const, get label() { return t().subtitles; } },
    { id: "shortcuts" as const, get label() { return t().shortcuts; } },
  ];

  function setLang(code: string) { settings.language = code; setLocale(code); settings.save(); }
  function resetVideo() { brightness = 0; contrast = 0; saturation = 0; zoom = 0; deinterlace = false; setBrightness(0); setContrast(0); setSaturation(0); resetVideoZoomPan(); }
  function applyEq() { setAudioEqualizer(eqBands).catch(() => {}); }
  function resetEq() { eqBands = [0,0,0,0,0]; resetAudioEqualizer(); }
  function setPreset(name: string) { eqBands = [...eqPresets[name]]; applyEq(); }

  function resetAll() {
    settings.resetAll();
    resetVideo();
    normEnabled = false; resetEq(); setAudioNormalization(false);
  }

  function close() { visible = false; }
</script>

{#if visible}
  <button aria-label="Close" class="fixed inset-0 z-90 w-full h-full bg-black/40 border-none cursor-default" onclick={close}></button>

  <div
    data-panel
    role="dialog"
    tabindex="-1"
    class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-91 w-130 h-[400px] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none overflow-hidden"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key === "Escape" && close()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-white/8">
      <span class="font-medium text-sm">{t().settings}</span>
      <button onclick={close} class="ctrl-btn w-7 h-7 text-xs">✕</button>
    </div>

    <div class="flex flex-1 min-h-0">
      <!-- Sidebar -->
      <div class="w-30 border-r border-white/8 py-2 flex-shrink-0">
        {#each tabs as tb}
          <button
            onclick={() => tab = tb.id}
            class="w-full text-left px-4 py-2 text-xs transition-colors {tab === tb.id ? 'bg-white/10 text-white font-medium' : 'text-white/50 hover:text-white/80 hover:bg-white/5'}"
          >{tb.label}</button>
        {/each}
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        {#if tab === "general"}
          <div class="s-group">
            <div class="s-row">
              <span>{t().language}</span>
              <Select items={Object.keys(languages)} value={settings.language} label={(code) => languages[code]} onchange={setLang} />
            </div>
          </div>
          <div class="s-group">
            <label class="s-row">
              <span>{t().defaultVolume}</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="100" bind:value={settings.volume} oninput={() => settings.save()} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right tabular-nums">{settings.volume}%</span>
              </div>
            </label>
            <div class="s-row">
              <span>{t().defaultSpeed}</span>
              <Select items={speeds} value={settings.speed} label={(s) => `${s}x`} onchange={(s) => { settings.speed = s; settings.save(); }} />
            </div>
          </div>
          <div class="s-group">
            <label class="s-toggle"><span>{t().rememberPosition}</span><input type="checkbox" bind:checked={settings.rememberPosition} onchange={() => settings.save()} /></label>
            <label class="s-toggle"><span>{t().autoPlay}</span><input type="checkbox" bind:checked={settings.autoPlay} onchange={() => settings.save()} /></label>
          </div>

        {:else if tab === "video"}
          <div class="s-group">
            <div class="flex items-center justify-between mb-1">
              <span class="text-white/50 text-xs uppercase tracking-wide">{t().color}</span>
              <button onclick={resetVideo} class="text-xs text-white/40 hover:text-white/70">{t().reset}</button>
            </div>
            {#each [
              { get label() { return t().brightness; }, get: () => brightness, set: (v: number) => { brightness = v; setBrightness(v); } },
              { get label() { return t().contrast; }, get: () => contrast, set: (v: number) => { contrast = v; setContrast(v); } },
              { get label() { return t().saturation; }, get: () => saturation, set: (v: number) => { saturation = v; setSaturation(v); } },
            ] as ctrl}
              <label class="s-row">
                <span>{ctrl.label}</span>
                <div class="flex items-center gap-2">
                  <input type="range" min="-100" max="100" value={ctrl.get()} oninput={(e) => ctrl.set(Number((e.target as HTMLInputElement).value))} class="s-range flex-1" />
                  <span class="text-white/50 w-8 text-right tabular-nums">{ctrl.get()}</span>
                </div>
              </label>
            {/each}
            <label class="s-row">
              <span>{t().zoom}</span>
              <div class="flex items-center gap-2">
                <input type="range" min="-1" max="3" step="0.1" bind:value={zoom} oninput={() => setVideoZoom(zoom)} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right tabular-nums">{zoom.toFixed(1)}</span>
              </div>
            </label>
          </div>
          <div class="s-group">
            <label class="s-toggle"><span>{t().deinterlace}</span><input type="checkbox" bind:checked={deinterlace} onchange={() => toggleDeinterlace()} /></label>
          </div>

        {:else if tab === "audio"}
          <div class="s-group">
            <label class="s-toggle"><span>{t().normalization}</span>
              <input type="checkbox" bind:checked={normEnabled} onchange={() => setAudioNormalization(normEnabled)} />
            </label>
          </div>
          <div class="s-group">
            <div class="flex items-center justify-between mb-1">
              <span class="text-white/50 text-xs uppercase tracking-wide">{t().equalizer}</span>
              <button onclick={resetEq} class="text-xs text-white/40 hover:text-white/70">{t().reset}</button>
            </div>
            <div class="flex gap-1 flex-wrap mb-2">
              {#each Object.keys(eqPresets) as name}
                <button class="px-2.5 py-1 rounded text-xs bg-white/[0.08] hover:bg-white/[0.15] text-white/70" onclick={() => setPreset(name)}>{name}</button>
              {/each}
            </div>
            {#each eqLabels as label, i}
              <div class="flex items-center gap-2">
                <span class="w-12 text-xs text-white/50 text-right">{label}</span>
                <input type="range" min="-12" max="12" step="1" bind:value={eqBands[i]} oninput={applyEq} class="s-range flex-1" />
                <span class="w-6 text-xs text-white/50 text-right tabular-nums">{eqBands[i] > 0 ? "+" : ""}{eqBands[i]}</span>
              </div>
            {/each}
          </div>

        {:else if tab === "subtitles"}
          <div class="s-group">
            <div class="s-row">
              <span>{t().font}</span>
              <div class="flex items-center gap-1.5">
                <div class="flex-1"><Select items={subFonts} value={settings.subFont} itemStyle={(f) => `font-family:'${f}'`} onchange={(f) => { settings.subFont = f; settings.applySubStyle(); }} /></div>
                <button
                  class="w-8 h-8 rounded text-sm font-bold border border-white/10 transition-all {settings.subBold ? 'bg-blue-500/20 text-blue-400 border-blue-400/30' : 'bg-white/8 text-white/60 hover:bg-white/12'}"
                  onclick={() => { settings.subBold = !settings.subBold; settings.applySubStyle(); }}
                >B</button>
              </div>
            </div>
            <label class="s-row">
              <span>{t().size}</span>
              <div class="flex items-center gap-2">
                <input type="range" min="10" max="100" bind:value={settings.subSize} oninput={() => settings.applySubStyle()} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right tabular-nums">{settings.subSize}</span>
              </div>
            </label>
            <div class="flex items-center justify-between py-1">
              <span>{t().textColor}</span>
              <input type="color" bind:value={settings.subColor} oninput={() => settings.applySubStyle()} class="s-color" />
            </div>
            <div class="flex items-center justify-between py-1">
              <span>{t().borderColor}</span>
              <input type="color" bind:value={settings.subBorderColor} oninput={() => settings.applySubStyle()} class="s-color" />
            </div>
            <label class="s-row">
              <span>{t().borderSize}</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="5" step="0.5" bind:value={settings.subBorderSize} oninput={() => settings.applySubStyle()} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right tabular-nums">{settings.subBorderSize}</span>
              </div>
            </label>
            <label class="s-row">
              <span>{t().position}</span>
              <div class="flex items-center gap-2">
                <input type="range" min="0" max="100" bind:value={settings.subPosition} oninput={() => settings.applySubStyle()} class="s-range flex-1" />
                <span class="text-white/50 w-8 text-right tabular-nums">{settings.subPosition}%</span>
              </div>
            </label>
          </div>
          <div class="s-group">
            <span class="text-white/50 text-xs uppercase tracking-wide">OpenSubtitles</span>
            <label class="s-row"><span>{t().apiKey}</span><input type="text" class="s-input" bind:value={settings.osApiKey} onchange={() => settings.save()} placeholder={t().apiKey} /></label>
            <label class="s-row"><span>{t().username}</span><input type="text" class="s-input" bind:value={settings.osUsername} onchange={() => settings.save()} placeholder={t().username} /></label>
            <label class="s-row"><span>{t().password}</span><input type="password" class="s-input" bind:value={settings.osPassword} onchange={() => settings.save()} placeholder={t().password} /></label>
          </div>
        {:else if tab === "shortcuts"}
          <div class="space-y-3">
            {#each [...new Set(keybindings.actions.map(a => a.category))] as category}
              <div>
                <span class="text-white/50 text-xs uppercase tracking-wide">{t()[category] ?? category}</span>
                {#each keybindings.actions.filter(a => a.category === category) as action}
                  <div class="s-row">
                    <span class="flex-1">{t()[action.i18nKey] ?? action.i18nKey}</span>
                    <button
                      class="min-w-[100px] px-3 py-1 text-xs rounded border transition-all text-center {rebinding === action.id ? 'border-blue-400 bg-blue-500/20 text-blue-400' : 'border-white/10 bg-white/5 text-white/80 hover:bg-white/10'}"
                      onclick={() => rebinding = action.id}
                      onkeydown={rebinding === action.id ? handleRebind : undefined}
                    >
                      {rebinding === action.id ? t().pressKey : keybindings.constructor.keyLabel(keybindings.getKey(action.id))}
                    </button>
                  </div>
                {/each}
              </div>
            {/each}
            <button class="w-full py-2 text-xs text-white/50 hover:text-white/80 border border-white/10 rounded hover:bg-white/5 transition-colors" onclick={() => { keybindings.resetAll(); settings.save(); }}>
              {t().resetShortcuts}
            </button>
          </div>
        {/if}
      </div>
    </div>

    <!-- Footer -->
    <div class="flex items-center px-4 py-2 border-t border-white/8">
      <button class="text-[11px] text-white/30 hover:text-white/60" onclick={resetAll}>{t().restoreDefaults}</button>
      <div class="flex-1"></div>
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
  .s-input { background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.1); border-radius: 6px; padding: 6px 8px; color: white; font-size: 12px; outline: none; }
  .s-input:focus { border-color: oklch(0.65 0.25 250); }
  .s-input::placeholder { color: rgba(255,255,255,0.25); }
</style>
