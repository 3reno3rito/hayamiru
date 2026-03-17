<script lang="ts">
  import { getTracks, selectSubtitle, loadSubtitle, toggleSubtitles, setSubtitleDelay, type TrackInfo } from "$lib/bindings/tracks";
  import { open } from "@tauri-apps/plugin-dialog";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let tracks = $state<TrackInfo[]>([]);
  let delay = $state(0);
  let subVisible = $state(true);

  async function refresh() {
    try {
      const all = await getTracks();
      tracks = all.filter((t) => t.track_type === "sub");
    } catch {}
  }

  $effect(() => { if (visible) refresh(); });

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
    <!-- Header -->
    <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
      <span class="font-medium text-xs">Subtitles</span>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => visible = false}>✕</button>
    </div>

    <!-- Track list -->
    <div class="flex-1 overflow-y-auto max-h-[180px]">
      <button
        class="w-full flex items-center px-3 py-2 hover:bg-white/[0.08] text-left {tracks.every(t => !t.selected) ? 'text-blue-400' : 'text-white/70'}"
        onclick={() => handleSelect(0)}
      >
        <span class="w-4 text-[10px] mr-2">{tracks.every((t: TrackInfo) => !t.selected) ? "✓" : "\u00A0"}</span>
        <span>Disabled</span>
      </button>

      {#each tracks as track}
        <button
          class="w-full flex items-center px-3 py-2 hover:bg-white/[0.08] text-left {track.selected ? 'text-blue-400' : 'text-white/70'}"
          onclick={() => handleSelect(track.id)}
        >
          <span class="w-4 text-[10px] mr-2">{track.selected ? "✓" : "\u00A0"}</span>
          <span class="flex-1 truncate">
            {track.title || track.lang || `Track ${track.id}`}
            {#if track.lang && track.title}
              <span class="text-white/30 ml-1">[{track.lang}]</span>
            {/if}
          </span>
        </button>
      {/each}

      {#if tracks.length === 0}
        <div class="px-3 py-4 text-center text-white/30 text-xs">No subtitle tracks</div>
      {/if}
    </div>

    <!-- Load external -->
    <div class="border-t border-white/[0.08]">
      <button
        class="w-full flex items-center justify-center gap-1.5 px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90"
        onclick={handleLoadExternal}
      >
        Load external file...
      </button>
    </div>

    <!-- Controls -->
    <div class="border-t border-white/[0.08] px-3 py-2 space-y-2">
      <div class="flex items-center justify-between">
        <span class="text-white/50">Visibility</span>
        <button
          class="px-2 py-0.5 rounded text-xs {subVisible ? 'bg-blue-500/30 text-blue-400' : 'bg-white/10 text-white/40'}"
          onclick={handleToggle}
        >{subVisible ? "ON" : "OFF"}</button>
      </div>

      <div class="flex items-center justify-between">
        <span class="text-white/50">Delay</span>
        <div class="flex items-center gap-1">
          <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(-0.1)}>-</button>
          <span class="w-14 text-center font-mono text-xs">{delay.toFixed(1)}s</span>
          <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(0.1)}>+</button>
        </div>
      </div>
    </div>
  </div>
{/if}
