<script lang="ts">
  import {
    getPlaylist, playlistAdd, playlistRemove, playlistPlayIndex,
    playlistClear, type PlaylistItem,
  } from "$lib/bindings/playlist";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { t } from "$lib/i18n/index.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let items = $state<PlaylistItem[]>([]);
  let repeatMode = $state<"off" | "all" | "one">("off");
  let shuffle = $state(false);

  function cycleRepeat() {
    repeatMode = repeatMode === "off" ? "all" : repeatMode === "all" ? "one" : "off";
    invoke("set_mpv_property", { name: "loop-playlist", value: repeatMode === "all" ? "inf" : "no" }).catch(() => {});
    invoke("set_mpv_property", { name: "loop-file", value: repeatMode === "one" ? "inf" : "no" }).catch(() => {});
  }

  async function toggleShuffle() {
    shuffle = !shuffle;
    try {
      await invoke("mpv_command", { args: shuffle ? ["playlist-shuffle"] : ["playlist-unshuffle"] });
    } catch {}
    await refresh();
  }

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
  <button aria-label="Close" class="fixed inset-0 z-[80] w-full h-full bg-transparent border-none cursor-default" onclick={() => visible = false}></button>

  <div data-panel class="fixed right-4 bottom-16 z-[81] w-[320px] max-h-[80vh] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 flex flex-col select-none">
    <!-- Header -->
    <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
      <span class="font-medium text-xs">{t().playlist}</span>
      <span class="text-white/30 text-[11px] ml-2">{items.length} items</span>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => visible = false}>✕</button>
    </div>

    <!-- Items -->
    <div class="flex-1 overflow-y-auto max-h-[400px]">
      {#each items as item, i}
        <div
          class="group w-full flex items-center px-3 py-2 hover:bg-white/[0.08] cursor-default {item.current ? 'bg-white/[0.05]' : ''}"
          role="option"
          aria-selected={item.current}
          tabindex="-1"
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

    <!-- Controls -->
    <div class="border-t border-white/[0.08] flex items-center px-2 py-1.5 gap-1">
      <button
        class="flex items-center justify-center w-7 h-7 rounded cursor-pointer border-none transition-all"
        style="background: {shuffle ? 'rgba(59,130,246,0.2)' : 'transparent'}; color: {shuffle ? 'rgb(96,165,250)' : 'rgba(255,255,255,0.5)'};"
        title="Shuffle" onclick={toggleShuffle}
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 3h5v5M4 20L21 3M21 16v5h-5M15 15l6 6M4 4l5 5"/></svg>
      </button>
      <button
        class="flex items-center justify-center w-7 h-7 rounded cursor-pointer border-none transition-all relative"
        style="background: {repeatMode !== 'off' ? 'rgba(59,130,246,0.2)' : 'transparent'}; color: {repeatMode !== 'off' ? 'rgb(96,165,250)' : 'rgba(255,255,255,0.5)'};"
        title="Repeat: {repeatMode}" onclick={cycleRepeat}
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 1l4 4-4 4M3 11V9a4 4 0 014-4h14M7 23l-4-4 4-4M21 13v2a4 4 0 01-4 4H3"/></svg>
        {#if repeatMode === "one"}<span class="absolute -bottom-0.5 -right-0.5 text-[7px] font-bold bg-blue-500 text-white rounded-full w-3 h-3 flex items-center justify-center">1</span>{/if}
      </button>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-7 h-7 rounded hover:bg-white/10 text-white/50 hover:text-white/90" title="Add files" onclick={handleAddFiles}>
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/></svg>
      </button>
      <button class="ctrl-btn w-7 h-7 rounded hover:bg-white/10 text-white/50 hover:text-white/90" title="Add folder" onclick={handleAddFolder}>
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/></svg>
      </button>
      <button class="ctrl-btn w-7 h-7 rounded hover:bg-white/10 text-white/50 hover:text-red-400" title="Clear" onclick={handleClear}>
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
      </button>
    </div>
  </div>
{/if}
