<script lang="ts">
  import { player } from "$lib/stores/player.svelte";
  import { togglePause, setSpeed } from "$lib/bindings/playback";
  import { toggleFullscreen } from "$lib/bindings/window";
  import SeekBar from "./SeekBar.svelte";
  import VolumeSlider from "./VolumeSlider.svelte";

  let { visible = true }: { visible?: boolean } = $props();

  const speeds = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 3.0, 4.0];

  function cycleSpeed() {
    const idx = speeds.indexOf(player.speed);
    const next = speeds[(idx + 1) % speeds.length];
    player.speed = next;
    setSpeed(next);
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

      <VolumeSlider />

      <span class="text-xs text-white/70 ml-2 font-mono tabular-nums">
        {player.formattedTime}
      </span>

      <div class="flex-1"></div>

      <button onclick={cycleSpeed} class="ctrl-btn h-7 px-2 text-xs font-medium" title="Playback speed">
        {player.speed}x
      </button>

      <button onclick={() => toggleFullscreen()} class="ctrl-btn w-8 h-8" title="Fullscreen (F)">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M4 8V4m0 0h4M4 4l5 5m11-5h-4m4 0v4m0-4l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5h-4m4 0v-4m0 4l-5-5" />
        </svg>
      </button>
    </div>
  </div>
{/if}
