<script lang="ts">
  import { getTracks, selectSubtitle, loadSubtitle, toggleSubtitles, setSubtitleDelay, type TrackInfo } from "$lib/bindings/tracks";
  import { translateSubtitles } from "$lib/bindings/translate";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import Select from "./Select.svelte";
  import { t } from "$lib/i18n/index.svelte";
  import { subStyle, subFonts } from "$lib/stores/subtitle-style.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let translateLang = $state("pt");
  let translating = $state(false);
  let translateProgress = $state(0);
  let translateTotal = $state(0);
  let translateError = $state("");

  const translateLangs = [
    { code: "pt", name: "Português" }, { code: "en", name: "English" }, { code: "es", name: "Español" },
    { code: "fr", name: "Français" }, { code: "de", name: "Deutsch" }, { code: "it", name: "Italiano" },
    { code: "ja", name: "日本語" }, { code: "ko", name: "한국어" }, { code: "zh", name: "中文" },
    { code: "ru", name: "Русский" }, { code: "ar", name: "العربية" }, { code: "hi", name: "हिन्दी" },
  ];

  async function handleTranslate() {
    translating = true; translateProgress = 0; translateTotal = 0; translateError = "";
    const unlisten = await listen<{ current: number; total: number; done: boolean }>("translate:progress", (e) => {
      translateProgress = e.payload.current;
      translateTotal = e.payload.total;
      if (e.payload.done) { translating = false; refresh(); }
    });
    try { await translateSubtitles(translateLang); }
    catch (e: any) { translateError = String(e); translating = false; }
    unlisten();
  }

  let tracks = $state<TrackInfo[]>([]);
  let delay = $state(0);
  let subVisible = $state(true);
  let page = $state<"main" | "style" | "translate">("main");


  async function refresh() {
    try {
      const all = await getTracks();
      tracks = all.filter((t) => t.track_type === "sub");
    } catch {}
  }

  $effect(() => { if (visible) { refresh(); page = "main"; } });

  async function handleSelect(id: number) {
    await selectSubtitle(id);
    await refresh();
  }

  async function handleLoadExternal() {
    const selected = await open({
      multiple: false,
      filters: [
        { name: "Subtitles", extensions: ["srt", "ass", "ssa", "sub", "vtt", "idx", "sup"] },
        { name: "All Files", extensions: ["*"] },
      ],
    });
    if (selected) {
      await loadSubtitle(selected as string);
      await refresh();
    }
  }

  function handleDelayChange(delta: number) {
    delay = +(delay + delta).toFixed(1);
    setSubtitleDelay(delay);
  }

  function handleToggle() {
    subVisible = !subVisible;
    toggleSubtitles();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[80]" onclick={() => visible = false}></div>

  <div data-panel class="fixed right-4 bottom-16 z-[81] w-[280px] max-h-[70vh] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none">

    {#if page === "main"}
      <!-- Header -->
      <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
        <span class="font-medium text-xs">{t().subtitles}</span>
        <div class="flex-1"></div>
        <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => visible = false}>✕</button>
      </div>

      <!-- Track list -->
      <div class="flex-1 overflow-y-auto max-h-[180px]">
        <button
          class="w-full flex items-center px-3 py-2 hover:bg-white/[0.08] text-left {tracks.every(t => !t.selected) ? 'text-blue-400' : 'text-white/70'}"
          onclick={() => handleSelect(0)}
        >
          <span class="w-4 text-xs mr-2">{tracks.every((t: TrackInfo) => !t.selected) ? "✓" : "\u00A0"}</span>
          <span>{t().disabled}</span>
        </button>

        {#each tracks as track}
          <button
            class="w-full flex items-center px-3 py-2 hover:bg-white/[0.08] text-left {track.selected ? 'text-blue-400' : 'text-white/70'}"
            onclick={() => handleSelect(track.id)}
          >
            <span class="w-4 text-xs mr-2">{track.selected ? "✓" : "\u00A0"}</span>
            <span class="flex-1 truncate">
              {track.title || track.lang || `Track ${track.id}`}
              {#if track.lang && track.title}
                <span class="text-white/30 ml-1">[{track.lang}]</span>
              {/if}
            </span>
          </button>
        {/each}

        {#if tracks.length === 0}
          <div class="px-3 py-4 text-center text-white/30 text-xs">{t().noSubtitleTracks}</div>
        {/if}
      </div>

      <!-- Load external -->
      <div class="border-t border-white/[0.08]">
        <button
          class="w-full flex items-center justify-center gap-1.5 px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90"
          onclick={handleLoadExternal}
        >
          {t().loadExternalFile}
        </button>
      </div>

      <!-- Controls -->
      <div class="border-t border-white/[0.08] px-3 py-2 space-y-2">
        <div class="flex items-center justify-between">
          <span class="text-white/50">{t().visibility}</span>
          <button
            class="px-2 py-0.5 rounded text-xs {subVisible ? 'bg-blue-500/30 text-blue-400' : 'bg-white/10 text-white/40'}"
            onclick={handleToggle}
          >{subVisible ? t().on : t().off}</button>
        </div>

        <div class="flex items-center justify-between">
          <span class="text-white/50">{t().delay}</span>
          <div class="flex items-center gap-1">
            <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(-0.1)}>-</button>
            <span class="w-14 text-center tabular-nums text-xs">{delay.toFixed(1)}s</span>
            <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(0.1)}>+</button>
          </div>
        </div>
      </div>

      <!-- Translate nav -->
      <div class="border-t border-white/[0.08]">
        <button class="w-full flex items-center justify-between px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90" onclick={() => page = "translate"}>
          <span>{t().translate || "Translate"}</span>
          <svg class="w-3.5 h-3.5 opacity-40" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </button>
      </div>

      <!-- Style nav -->
      <div class="border-t border-white/[0.08]">
        <button class="w-full flex items-center justify-between px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90" onclick={() => page = "style"}>
          <span>{t().style}</span>
          <svg class="w-3.5 h-3.5 opacity-40" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
        </button>
      </div>

    {:else if page === "translate"}
      <!-- Translate header -->
      <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
        <button class="ctrl-btn w-6 h-6 text-xs mr-2 hover:bg-white/10 rounded" onclick={() => page = "main"}>←</button>
        <span class="font-medium text-xs">{t().translate || "Translate"}</span>
      </div>

      <!-- Translate controls -->
      <div class="flex-1 overflow-y-auto p-3 space-y-3">
        {#if translating}
          <div class="space-y-2">
            <div class="w-full h-1.5 bg-white/10 rounded-full overflow-hidden">
              <div class="h-full bg-blue-500 rounded-full transition-all" style="width: {translateTotal > 0 ? (translateProgress / translateTotal) * 100 : 0}%"></div>
            </div>
            <span class="text-white/30 text-xs">{translateProgress}/{translateTotal}</span>
          </div>
        {:else}
          <button class="w-full py-2 rounded text-xs bg-blue-500/20 text-blue-400 hover:bg-blue-500/30" onclick={handleTranslate}>
            {t().translateNow || "Translate"}
          </button>
        {/if}

        {#if translateError}
          <span class="text-red-400 text-xs truncate block" title={translateError}>{translateError}</span>
        {/if}

        <div>
          <span class="text-white/50 text-xs block mb-1">{t().to || "To"}</span>
          <Select items={translateLangs.map(l => l.name)} value={translateLangs.find(l => l.code === translateLang)?.name || "Português"} onchange={(name) => { const l = translateLangs.find(x => x.name === name); if (l) translateLang = l.code; }} />
        </div>
      </div>

    {:else if page === "style"}
      <!-- Style header -->
      <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
        <button class="ctrl-btn w-6 h-6 text-xs mr-2 hover:bg-white/10 rounded" onclick={() => page = "main"}>←</button>
        <span class="font-medium text-xs">{t().style}</span>
        <div class="flex-1"></div>
        <button class="text-xs text-white/40 hover:text-white/70" onclick={() => subStyle.reset()}>{t().reset}</button>
      </div>

      <!-- Style controls -->
      <div class="flex-1 overflow-y-auto p-3 space-y-3">
        <div>
          <span class="text-white/50 text-xs block mb-1">{t().font}</span>
          <Select items={subFonts} value={subStyle.font} itemStyle={(f) => `font-family:'${f}'`} onchange={(f) => { subStyle.font = f; subStyle.apply(); }} />
        </div>

        <div>
          <div class="flex items-center justify-between mb-1">
            <span class="text-white/50 text-xs">{t().size}</span>
            <span class="text-white/50 text-xs tabular-nums">{subStyle.size}</span>
          </div>
          <input type="range" min="20" max="100" bind:value={subStyle.size} oninput={() => subStyle.apply()} class="s-range w-full" />
        </div>

        <div class="flex items-center gap-4">
          <div class="flex items-center gap-2">
            <span class="text-white/50 text-xs">{t().text}</span>
            <input type="color" bind:value={subStyle.color} oninput={() => subStyle.apply()} class="s-color" />
          </div>
          <div class="flex items-center gap-2">
            <span class="text-white/50 text-xs">{t().border}</span>
            <input type="color" bind:value={subStyle.borderColor} oninput={() => subStyle.apply()} class="s-color" />
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-1">
            <span class="text-white/50 text-xs">{t().borderSize}</span>
            <span class="text-white/50 text-xs tabular-nums">{subStyle.borderSize}</span>
          </div>
          <input type="range" min="0" max="10" bind:value={subStyle.borderSize} oninput={() => subStyle.apply()} class="s-range w-full" />
        </div>

        <div>
          <div class="flex items-center justify-between mb-1">
            <span class="text-white/50 text-xs">{t().position}</span>
            <span class="text-white/50 text-xs tabular-nums">{subStyle.position}%</span>
          </div>
          <input type="range" min="0" max="100" bind:value={subStyle.position} oninput={() => subStyle.apply()} class="s-range w-full" />
        </div>
      </div>
    {/if}
  </div>
{/if}
