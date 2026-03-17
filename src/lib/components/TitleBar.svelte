<script lang="ts">
  import { minimizeWindow, maximizeWindow, closeWindow } from "$lib/bindings/window";
  import { player } from "$lib/stores/player.svelte";

  let { visible = true, menuOpen = false, onmenu }: { visible?: boolean; menuOpen?: boolean; onmenu?: (x: number, y: number) => void } = $props();
</script>

{#if visible}
  <div
    class="fixed top-0 left-0 right-0 z-50 flex items-center h-9 px-3 drag-region controls-overlay bg-gradient-to-b from-black/70 to-transparent"
  >
    <button onclick={(e) => { e.stopPropagation(); if (menuOpen) { onmenu?.(-1, -1); } else { const r = (e.currentTarget as HTMLElement).getBoundingClientRect(); onmenu?.(r.left, r.bottom + 4); } }} class="ctrl-btn w-7 h-7 no-drag mr-2" title="Menu">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
        <path stroke-linecap="round" d="M4 6h16M4 12h16M4 18h16" />
      </svg>
    </button>

    <span class="text-xs text-white/80 truncate flex-1 pointer-events-none">
      {player.title || "Hayamiru"}
    </span>

    <div class="flex items-center gap-0.5 no-drag">
      <button onclick={() => minimizeWindow()} class="ctrl-btn w-8 h-7" title="Minimize">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M5 12h14" />
        </svg>
      </button>
      <button onclick={() => maximizeWindow()} class="ctrl-btn w-8 h-7" title="Maximize">
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <rect x="3" y="3" width="18" height="18" rx="2" stroke-width="2" />
        </svg>
      </button>
      <button onclick={() => closeWindow()} class="ctrl-btn w-8 h-7 hover:!bg-red-500/80" title="Close">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
{/if}
