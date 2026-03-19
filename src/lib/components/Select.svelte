<script lang="ts" generics="T">
  interface Props {
    items: T[];
    value: T;
    label?: (item: T) => string;
    itemStyle?: (item: T) => string;
    onchange: (item: T) => void;
  }

  let { items, value, label, itemStyle, onchange }: Props = $props();
  let open = $state(false);

  function toggle(e: MouseEvent) {
    e.stopPropagation();
    open = !open;
  }

  function select(item: T) {
    onchange(item);
    open = false;
  }

  function display(item: T): string {
    if (label) return label(item);
    return String(item);
  }

  function onWindowClick() {
    if (open) open = false;
  }
</script>

<svelte:window onclick={onWindowClick} />

<div class="relative">
  <button
    class="s-drop-btn {open ? 'text-blue-400' : ''}"
    onclick={toggle}
  >
    {display(value)}
    <svg class="w-4 h-4 opacity-40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
    </svg>
  </button>
  {#if open}
    <div class="s-drop-list" role="listbox" tabindex="-1" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.key === "Escape" && (open = false)}>
      {#each items as item}
        <button
          class="s-drop-item {item === value ? 'text-blue-400' : 'text-white/80'}"
          style={itemStyle?.(item) ?? ""}
          onclick={() => select(item)}
        >{display(item)}</button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .s-drop-btn {
    width: 100%; display: flex; align-items: center; justify-content: space-between;
    background: rgba(255,255,255,0.08); border: 1px solid rgba(255,255,255,0.1);
    border-radius: 6px; color: rgba(255,255,255,0.9); padding: 6px 10px;
    font-size: 13px;
  }
  .s-drop-btn:hover { background: rgba(255,255,255,0.12); }
  .s-drop-list {
    position: absolute; left: 0; right: 0; top: 100%; margin-top: 4px; z-index: 10;
    max-height: 200px; overflow-y: auto;
    background: #1a1a1f; border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.5); padding: 4px 0;
  }
  .s-drop-item {
    width: 100%; text-align: left; padding: 6px 12px; font-size: 13px;
    background: none; border: none;
  }
  .s-drop-item:hover { background: rgba(255,255,255,0.1); }
</style>
