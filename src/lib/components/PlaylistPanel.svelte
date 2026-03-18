<script lang="ts">
  import {
    getPlaylist, playlistAdd, playlistRemove, playlistPlayIndex,
    playlistClear, type PlaylistItem,
  } from "$lib/bindings/playlist";
  import { open } from "@tauri-apps/plugin-dialog";
  import { t } from "$lib/i18n/index.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let items = $state<PlaylistItem[]>([]);

  async function refresh() {
    try { items = await getPlaylist(); } catch {}
  }

  $effect(() => { if (visible) refresh(); });

  async function handlePlay(index: number) {
    await playlistPlayIndex(index);
    await refresh();
  }

  async function handleRemove(index: number) {
    await playlistRemove(index);
    await refresh();
  }

  async function handleAddFiles() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Media Files",
          extensions: [
            "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm",
            "mpg", "mpeg", "m4v", "3gp", "ts", "vob",
            "mp3", "flac", "wav", "ogg", "m4a", "aac", "opus", "wma",
          ],
        },
        { name: "All Files", extensions: ["*"] },
      ],
    });
    if (selected && Array.isArray(selected)) {
      for (const path of selected) await playlistAdd(path);
      await refresh();
    } else if (selected) {
      await playlistAdd(selected as string);
      await refresh();
    }
  }

  async function handleAddFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      // Scan folder for media files via Rust, or add all files manually
      // For now, we rely on mpv's ability to handle directories
      await playlistAdd(selected as string);
      await refresh();
    }
  }

  async function handleClear() {
    await playlistClear();
    await refresh();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[80]" onclick={() => visible = false}></div>

  <div data-panel class="fixed right-4 bottom-16 z-[81] w-[320px] max-h-[80vh] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none">
    <!-- Header -->
    <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
      <span class="font-medium text-xs">{t().playlist}</span>
      <span class="text-white/30 text-[11px] ml-2">{items.length} items</span>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-6 h-6 text-[11px] hover:text-red-400" title="Clear" onclick={handleClear}>🗑</button>
      <button class="ctrl-btn w-6 h-6 text-xs ml-1" onclick={() => visible = false}>✕</button>
    </div>

    <!-- Items -->
    <div class="flex-1 overflow-y-auto max-h-[400px]">
      {#each items as item, i}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="group w-full flex items-center px-3 py-2 hover:bg-white/[0.08] cursor-default {item.current ? 'bg-white/[0.05]' : ''}"
          ondblclick={() => handlePlay(item.index)}
          title={item.filename}
        >
          <span class="w-5 text-[11px] shrink-0 {item.current ? 'text-blue-400' : 'text-white/25'}">
            {item.current ? "▶" : i + 1}
          </span>

          <span class="flex-1 truncate mx-2 {item.current ? 'text-blue-400' : 'text-white/80'}">
            {item.title}
          </span>

          <button
            class="ctrl-btn w-5 h-5 opacity-0 group-hover:opacity-100 hover:text-red-400 transition-opacity"
            title="Remove"
            onclick={(e) => { e.stopPropagation(); handleRemove(item.index); }}
          >✕</button>
        </div>
      {/each}

      {#if items.length === 0}
        <div class="px-3 py-8 text-center text-white/30 text-xs">{t().emptyPlaylist}</div>
      {/if}
    </div>

    <!-- Actions -->
    <div class="border-t border-white/[0.08] flex">
      <button
        class="flex-1 flex items-center justify-center gap-1 px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90"
        onclick={handleAddFiles}
      >{t().addFiles}</button>
      <div class="w-px bg-white/[0.08]"></div>
      <button
        class="flex-1 flex items-center justify-center gap-1 px-3 py-2 hover:bg-white/[0.08] text-white/60 hover:text-white/90"
        onclick={handleAddFolder}
      >{t().addFolder}</button>
    </div>
  </div>
{/if}
