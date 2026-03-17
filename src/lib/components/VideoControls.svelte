<script lang="ts">
  import { player } from "$lib/stores/player.svelte";
  import { togglePause, setSpeed } from "$lib/bindings/playback";
  import { playlistNext, playlistPrev } from "$lib/bindings/playlist";
  import { toggleFullscreen } from "$lib/bindings/window";
  import SeekBar from "./SeekBar.svelte";
  import VolumeSlider from "./VolumeSlider.svelte";
  import SubtitlePanel from "./SubtitlePanel.svelte";
  import AudioPanel from "./AudioPanel.svelte";
  import PlaylistPanel from "./PlaylistPanel.svelte";

  let { visible = true }: { visible?: boolean } = $props();

  let subPanelVisible = $state(false);
  let audioPanelVisible = $state(false);
  let playlistPanelVisible = $state(false);
  let speedDropOpen = $state(false);
  let speedBtnEl = $state<HTMLButtonElement | null>(null);

  const speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];

  function closeAll() {
    subPanelVisible = false;
    audioPanelVisible = false;
    playlistPanelVisible = false;
    speedDropOpen = false;
  }

  function togglePanel(panel: "sub" | "audio" | "playlist") {
    const map = { sub: () => subPanelVisible, audio: () => audioPanelVisible, playlist: () => playlistPanelVisible };
    const was = map[panel]();
    closeAll();
    if (panel === "sub") subPanelVisible = !was;
    else if (panel === "audio") audioPanelVisible = !was;
    else playlistPanelVisible = !was;
  }

  function pickSpeed(s: number) {
    player.speed = s;
    setSpeed(s);
    speedDropOpen = false;
  }
</script>

{#if visible}
  <div class="fixed bottom-0 left-0 right-0 z-50 controls-overlay bg-gradient-to-t from-black/80 to-transparent pt-8 pb-2 px-4">
    <div class="mb-2"><SeekBar /></div>

    <div class="flex items-center gap-1">
      <button onclick={() => togglePause()} class="ctrl-btn w-9 h-9" title={player.playing ? "Pause" : "Play"}>
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          {#if player.playing}
            <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
          {:else}
            <path d="M8 5v14l11-7z" />
          {/if}
        </svg>
      </button>

      <!-- Previous -->
      <button onclick={() => playlistPrev().catch(() => {})} class="ctrl-btn w-8 h-8" title="Previous (P)">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
          <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
        </svg>
      </button>

      <!-- Next -->
      <button onclick={() => playlistNext().catch(() => {})} class="ctrl-btn w-8 h-8" title="Next (N)">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
          <path d="M16 6h2v12h-2zm-10 6l8.5 6V6z" transform="scale(-1,1) translate(-24,0)" />
        </svg>
      </button>

      <VolumeSlider />

      <span class="text-xs text-white/70 ml-2 font-mono tabular-nums">
        {player.formattedTime}
      </span>

      <div class="flex-1"></div>

      <!-- Subtitle -->
      <button onclick={() => togglePanel("sub")} class="ctrl-btn w-8 h-8 {subPanelVisible ? 'text-blue-400' : ''}" title="Subtitles (V)">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <rect x="2" y="5" width="20" height="14" rx="2" stroke-width="2" />
          <path stroke-width="2" d="M6 13h4M14 13h4M8 17h8" />
        </svg>
      </button>

      <!-- Audio -->
      <button onclick={() => togglePanel("audio")} class="ctrl-btn w-8 h-8 {audioPanelVisible ? 'text-blue-400' : ''}" title="Audio track (A)">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2z" />
        </svg>
      </button>

      <!-- Speed -->
      <button
        bind:this={speedBtnEl}
        onclick={() => { closeAll(); speedDropOpen = !speedDropOpen; }}
        class="ctrl-btn h-7 px-2 text-xs font-medium {speedDropOpen ? 'text-blue-400' : ''}"
        title="Playback speed"
      >
        {player.speed}x
      </button>

      <!-- Playlist -->
      <button onclick={() => togglePanel("playlist")} class="ctrl-btn w-8 h-8 {playlistPanelVisible ? 'text-blue-400' : ''}" title="Playlist">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h10M4 18h10M18 14v6M15 17h6" />
        </svg>
      </button>

      <!-- Fullscreen -->
      <button onclick={() => toggleFullscreen()} class="ctrl-btn w-8 h-8" title="Fullscreen (F)">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M4 8V4m0 0h4M4 4l5 5m11-5h-4m4 0v4m0-4l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5h-4m4 0v-4m0 4l-5-5" />
        </svg>
      </button>
    </div>
  </div>
{/if}

<!-- Speed dropdown -->
{#if speedDropOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[70]" onclick={() => speedDropOpen = false}></div>
  <div
    data-panel
    class="fixed z-[71] w-[100px] bg-[#1a1a1f]/98 backdrop-blur-md border border-white/10 rounded-lg shadow-2xl py-1"
    style="bottom: {speedBtnEl ? window.innerHeight - speedBtnEl.getBoundingClientRect().top + 4 : 60}px; left: {speedBtnEl ? speedBtnEl.getBoundingClientRect().left : 0}px;"
  >
    {#each speeds as s}
      <button
        class="w-full text-left px-3 py-1.5 text-[13px] hover:bg-white/10 {player.speed === s ? 'text-blue-400' : 'text-white/80'}"
        onclick={() => pickSpeed(s)}
      >
        {player.speed === s ? "✓" : "\u00A0"}
        {s}x
      </button>
    {/each}
  </div>
{/if}

<!-- Panels -->
<SubtitlePanel bind:visible={subPanelVisible} />
<AudioPanel bind:visible={audioPanelVisible} />
<PlaylistPanel bind:visible={playlistPanelVisible} />
