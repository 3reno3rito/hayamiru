<script lang="ts">
  import { player } from "$lib/stores/player.svelte";
  import { setVolume } from "$lib/bindings/playback";

  let sliderEl: HTMLDivElement;
  let dragging = $state(false);

  function handleVolume(e: MouseEvent) {
    if (!sliderEl) return;
    const rect = sliderEl.getBoundingClientRect();
    const vol = Math.round(Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width)) * 100);
    setVolume(vol);
    player.volume = vol;
    player.muted = vol === 0;
  }

  function toggleMute() {
    player.muted = !player.muted;
    setVolume(player.muted ? 0 : player.volume || 100);
  }
</script>

<svelte:window onmouseup={() => (dragging = false)} onmousemove={(e) => dragging && handleVolume(e)} />

<div class="flex items-center gap-1.5 group">
  <button onclick={toggleMute} class="ctrl-btn w-7 h-7" title={player.muted ? "Unmute" : "Mute"}>
    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      {#if player.muted || player.volume === 0}
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
        <path stroke-linecap="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
      {:else}
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M15.536 8.464a5 5 0 010 7.072M17.95 6.05a8 8 0 010 11.9" />
      {/if}
    </svg>
  </button>

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="relative w-20 h-1 bg-white/20 rounded cursor-pointer group-hover:h-1.5 transition-all"
    bind:this={sliderEl}
    onmousedown={(e) => { dragging = true; handleVolume(e); }}
  >
    <div
      class="absolute top-0 left-0 h-full bg-white rounded pointer-events-none"
      style="width: {player.muted ? 0 : player.volume}%"
    ></div>
  </div>
</div>
