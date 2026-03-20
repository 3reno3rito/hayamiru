<script lang="ts">
  import { player } from "$lib/stores/player.svelte";
  import { t } from "$lib/i18n/index.svelte";
  import { togglePause, setSpeed } from "$lib/bindings/playback";
  import { playlistNext, playlistPrev } from "$lib/bindings/playlist";
  import { toggleFullscreen } from "$lib/bindings/window";
  import SeekBar from "./SeekBar.svelte";
  import VolumeSlider from "./VolumeSlider.svelte";
  import SubtitlePanel from "./SubtitlePanel.svelte";
  import AudioPanel from "./AudioPanel.svelte";
  import PlaylistPanel from "./PlaylistPanel.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";

  let { visible = true, settingsOpen = $bindable(false) }: { visible?: boolean; settingsOpen?: boolean } = $props();

  let subPanelVisible = $state(false);
  let audioPanelVisible = $state(false);
  let playlistPanelVisible = $state(false);
  let settingsPanelVisible = $state(false);
  $effect(() => { if (settingsOpen) { closeAll(); settingsPanelVisible = true; settingsOpen = false; } });
  let speedDropOpen = $state(false);
  let speedBtnEl = $state<HTMLButtonElement | null>(null);

  const speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];

  function closeAll() {
    subPanelVisible = false;
    audioPanelVisible = false;
    playlistPanelVisible = false;
    settingsPanelVisible = false;
    speedDropOpen = false;
  }

  function togglePanel(panel: "sub" | "audio" | "playlist" | "settings") {
    const map = { sub: () => subPanelVisible, audio: () => audioPanelVisible, playlist: () => playlistPanelVisible, settings: () => settingsPanelVisible };
    const was = map[panel]();
    closeAll();
    if (panel === "sub") subPanelVisible = !was;
    else if (panel === "audio") audioPanelVisible = !was;
    else if (panel === "playlist") playlistPanelVisible = !was;
    else settingsPanelVisible = !was;
  }

  function pickSpeed(s: number) {
    player.speed = s;
    setSpeed(s);
    speedDropOpen = false;
  }
</script>

{#if visible}
  <div class="fixed bottom-0 left-0 right-0 z-50 controls-overlay bg-gradient-to-t from-black/80 to-transparent pt-6 pb-2 px-4">
    <div class="mb-1"><SeekBar /></div>

    <div class="flex items-center gap-1">
      <button onclick={() => { if (player.duration > 0) player.playing = !player.playing; togglePause(); }} class="ctrl-btn w-9 h-9" title={player.playing ? `${t().pause} (Space)` : `${t().play} (Space)`}>
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          {#if player.playing}
            <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z" />
          {:else}
            <path d="M8 5v14l11-7z" />
          {/if}
        </svg>
      </button>

      <!-- Previous -->
      <button onclick={() => playlistPrev().catch(() => {})} class="ctrl-btn w-8 h-8" title="{t().previous} (P)">
        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
          <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
        </svg>
      </button>

      <!-- Next -->
      <button onclick={() => playlistNext().catch(() => {})} class="ctrl-btn w-8 h-8" title="{t().next} (N)">
        <svg class="w-4 h-4 scale-x-[-1]" fill="currentColor" viewBox="0 0 24 24">
          <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
        </svg>
      </button>

      <VolumeSlider />

      <span class="text-xs text-white/70 ml-2 tabular-nums">
        {player.formattedTime}
      </span>

      <div class="flex-1"></div>

      <!-- Subtitle -->
      <button onclick={() => togglePanel("sub")} class="ctrl-btn w-8 h-8 {subPanelVisible ? 'text-blue-400' : ''}" title="{t().subtitles} (V)">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <rect x="2" y="5" width="20" height="14" rx="2" stroke-width="2" />
          <path stroke-width="2" d="M6 13h4M14 13h4M8 17h8" />
        </svg>
      </button>

      <!-- Audio -->
      <button onclick={() => togglePanel("audio")} class="ctrl-btn w-8 h-8 {audioPanelVisible ? 'text-blue-400' : ''}" title="{t().audio} (A)">
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
        title="{t().playbackSpeed}"
      >
        {player.speed}x
      </button>

      <!-- Playlist -->
      <button onclick={() => togglePanel("playlist")} class="ctrl-btn w-8 h-8 {playlistPanelVisible ? 'text-blue-400' : ''}" title="{t().playlist}">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h10M4 18h10M18 14v6M15 17h6" />
        </svg>
      </button>

      <!-- Settings -->
      <button onclick={() => togglePanel("settings")} class="ctrl-btn w-8 h-8 {settingsPanelVisible ? 'text-blue-400' : ''}" title="{t().settings}">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <circle cx="12" cy="12" r="3" stroke-width="2" />
        </svg>
      </button>

      <!-- Fullscreen -->
      <button onclick={() => toggleFullscreen()} class="ctrl-btn w-8 h-8" title="{t().fullscreen} (F)">
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
  <button aria-label="Close" class="fixed inset-0 z-70 w-full h-full bg-transparent border-none cursor-default" onclick={() => speedDropOpen = false}></button>
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
<SettingsPanel bind:visible={settingsPanelVisible} />
