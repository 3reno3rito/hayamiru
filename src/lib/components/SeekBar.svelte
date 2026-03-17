<script lang="ts">
  import { player } from "$lib/stores/player.svelte";
  import { seekAbsolute } from "$lib/bindings/playback";

  let trackEl: HTMLDivElement;
  let seeking = $state(false);

  function handleSeek(e: MouseEvent) {
    if (!trackEl || player.duration <= 0) return;
    const rect = trackEl.getBoundingClientRect();
    const fraction = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    seekAbsolute(fraction * player.duration);
  }

  function onMouseDown(e: MouseEvent) {
    seeking = true;
    handleSeek(e);
  }
  function onMouseMove(e: MouseEvent) {
    if (seeking) handleSeek(e);
  }
  function onMouseUp() {
    seeking = false;
  }
</script>

<svelte:window onmouseup={onMouseUp} onmousemove={onMouseMove} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="seek-track w-full" bind:this={trackEl} onmousedown={onMouseDown}>
  <div class="seek-progress" style="width: {player.progress}%"></div>
  <div class="seek-thumb" style="left: {player.progress}%"></div>
</div>
