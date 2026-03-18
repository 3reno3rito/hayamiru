<script lang="ts">
  import { getTracks, selectAudioTrack, setAudioDelay, type TrackInfo } from "$lib/bindings/tracks";
  import { t } from "$lib/i18n/index.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let tracks = $state<TrackInfo[]>([]);
  let delay = $state(0);

  async function refresh() {
    try {
      const all = await getTracks();
      tracks = all.filter((t) => t.track_type === "audio");
    } catch {}
  }

  $effect(() => { if (visible) refresh(); });

  async function handleSelect(id: number) {
    await selectAudioTrack(id);
    await refresh();
  }

  function handleDelayChange(delta: number) {
    delay = +(delay + delta).toFixed(1);
    setAudioDelay(delay);
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[80]" onclick={() => visible = false}></div>

  <div data-panel class="fixed right-4 bottom-16 z-[81] w-[280px] max-h-[60vh] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none">
    <!-- Header -->
    <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
      <span class="font-medium text-xs">{t().audio}</span>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => visible = false}>✕</button>
    </div>

    <!-- Track list -->
    <div class="flex-1 overflow-y-auto max-h-[250px]">
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
        <div class="px-3 py-4 text-center text-white/30 text-xs">{t().noAudioTracks}</div>
      {/if}
    </div>

    <!-- Delay control -->
    <div class="border-t border-white/[0.08] px-3 py-2">
      <div class="flex items-center justify-between">
        <span class="text-white/50">{t().delay}</span>
        <div class="flex items-center gap-1">
          <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(-0.1)}>-</button>
          <span class="w-14 text-center tabular-nums text-xs">{delay.toFixed(1)}s</span>
          <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => handleDelayChange(0.1)}>+</button>
        </div>
      </div>
    </div>
  </div>
{/if}
