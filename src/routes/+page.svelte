<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { player } from "$lib/stores/player.svelte";
  import { settings } from "$lib/stores/settings.svelte";
  import { t } from "$lib/i18n/index.svelte";
  import {
    initPlayer, openFile, togglePause, seekRelative, setVolume, setSpeed, getPlaybackState,
    screenshot, frameStep, frameBackStep, toggleAbLoop,
  } from "$lib/bindings/playback";
  import { setAspectRatio, getAspectRatio } from "$lib/bindings/video";
  import { toggleFullscreen } from "$lib/bindings/window";
  import { getTracks, selectSubtitle, selectAudioTrack } from "$lib/bindings/tracks";
  import { playlistNext, playlistPrev } from "$lib/bindings/playlist";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import VideoControls from "$lib/components/VideoControls.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import MediaInfoPanel from "$lib/components/MediaInfoPanel.svelte";

  let fileLoaded = $state(false);
  let dragOver = $state(false);
  let ctxShow = $state(false);
  let ctxX = $state(0);
  let ctxY = $state(0);
  let infoPanel = $state(false);
  let settingsOpen = $state(false);

  function openPanel(name: string) {
    if (name === "info") infoPanel = true;
    if (name === "settings") settingsOpen = true;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!player.fullscreen) { player.controlsVisible = true; return; }
    if (document.querySelector("[data-panel]")) { player.controlsVisible = true; return; }
    player.controlsVisible = e.clientY <= 50 || e.clientY >= window.innerHeight - 80;
  }

  $effect(() => {
    if (!player.fullscreen) player.controlsVisible = true;
  });

  $effect(() => {
    const cleanups: Array<Promise<() => void> | (() => void)> = [];

    initPlayer().catch(() => {});
    settings.load();
    getCurrentWebviewWindow().show();

    // Event-driven state updates
    cleanups.push(listen<number>("mpv:time-pos", (e) => { player.currentTime = e.payload; }));
    cleanups.push(listen<number>("mpv:duration", (e) => { player.duration = e.payload; }));
    cleanups.push(listen<boolean>("mpv:pause", (e) => { if (player.duration > 0) player.playing = !e.payload; }));
    cleanups.push(listen<number>("mpv:volume", (e) => { player.volume = e.payload; }));
    cleanups.push(listen<string>("mpv:media-title", (e) => { player.title = e.payload; }));
    cleanups.push(listen<void>("mpv:end-file", () => { player.playing = false; }));
    cleanups.push(listen<void>("mpv:file-loaded", () => { settings.applySubStyle(); }));

    // Open files from CLI args ("Open with" from Explorer)
    cleanups.push(listen<string[]>("open-files", (e) => {
      if (e.payload.length > 0) { fileLoaded = true; openFile(e.payload[0]); }
    }));

    // Polling fallback
    const poll = setInterval(() => {
      getPlaybackState().then((s) => {
        player.currentTime = s.time_pos;
        player.duration = s.duration;
        player.playing = s.duration > 0 && !s.paused;
        player.title = s.title;
        player.volume = s.volume;
      }).catch(() => {});
    }, 1000);
    cleanups.push(() => clearInterval(poll));

    // Drag & drop
    getCurrentWebviewWindow().onDragDropEvent((event) => {
      if (event.payload.type === "enter" || event.payload.type === "over") {
        dragOver = true;
      } else if (event.payload.type === "leave") {
        dragOver = false;
      } else if (event.payload.type === "drop") {
        dragOver = false;
        const paths = event.payload.paths;
        if (paths.length > 0) {
          fileLoaded = true;
          openFile(paths[0]);
        }
      }
    }).then((fn) => cleanups.push(fn));

    return () => {
      for (const c of cleanups) {
        if (typeof c === "function") c();
        else c.then((fn) => fn());
      }
    };
  });

  const ratioList = ["-1", "16:9", "4:3", "21:9", "2.35:1"];
  async function cycleRatio() {
    try {
      const current = await getAspectRatio();
      const idx = ratioList.indexOf(current);
      const next = ratioList[(idx + 1) % ratioList.length];
      await setAspectRatio(next);
    } catch {}
  }

  async function cycleTrack(type: "sub" | "audio") {
    try {
      const tracks = await getTracks();
      const filtered = tracks.filter((t) => t.track_type === type);
      if (filtered.length === 0) return;
      const current = filtered.find((t) => t.selected);
      const idx = current ? filtered.indexOf(current) : -1;
      const next = filtered[(idx + 1) % filtered.length];
      if (type === "sub") selectSubtitle(next.id);
      else selectAudioTrack(next.id);
    } catch {}
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement) return;

    switch (e.key) {
      case " ": e.preventDefault(); if (player.duration > 0) player.playing = !player.playing; togglePause(); break;
      case "f": case "F": case "F11": e.preventDefault(); toggleFullscreen(); break;
      case "ArrowRight": seekRelative(e.shiftKey ? 30 : 5); break;
      case "ArrowLeft": seekRelative(e.shiftKey ? -30 : -5); break;
      case "ArrowUp": e.preventDefault(); player.volume = Math.min(100, player.volume + 5); setVolume(player.volume); break;
      case "ArrowDown": e.preventDefault(); player.volume = Math.max(0, player.volume - 5); setVolume(player.volume); break;
      case "m": case "M": player.muted = !player.muted; setVolume(player.muted ? 0 : player.volume); break;
      case "v": case "V": cycleTrack("sub"); break;
      case "a": case "A": cycleTrack("audio"); break;
      case "n": case "N": playlistNext().catch(() => {}); break;
      case "p": case "P": playlistPrev().catch(() => {}); break;
      case "s": case "S": screenshot().catch(() => {}); break;
      case ".": frameStep().catch(() => {}); break;
      case ",": frameBackStep().catch(() => {}); break;
      case "r": case "R": cycleRatio(); break;
      case "l": case "L": toggleAbLoop().catch(() => {}); break;
      case "+": case "=": player.speed = Math.min(4, +(player.speed + 0.25).toFixed(2)); setSpeed(player.speed); break;
      case "-": player.speed = Math.max(0.25, +(player.speed - 0.25).toFixed(2)); setSpeed(player.speed); break;
      case "i": case "I": openPanel("info"); break;
      case "Escape":
        if (infoPanel) { infoPanel = false; break; }
        if (player.fullscreen) toggleFullscreen();
        break;
    }

    if (e.ctrlKey && e.key === "o") {
      e.preventDefault();
      handleOpenFile();
    }
  }

  async function handleOpenFile() {
    const selected = await open({
      multiple: false,
      filters: [
        { name: "Video", extensions: ["mp4","mkv","avi","mov","wmv","flv","webm","mpg","mpeg","m4v","3gp","ts","vob"] },
        { name: "Audio", extensions: ["mp3","flac","wav","ogg","m4a","aac","opus","wma"] },
        { name: "All", extensions: ["*"] },
      ],
    });
    if (selected) {
      fileLoaded = true;
      openFile(selected as string);
    }
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    ctxX = e.clientX;
    ctxY = e.clientY;
    ctxShow = true;
  }

  function handleDoubleClick(e: MouseEvent) {
    const el = e.target as HTMLElement;
    // Only toggle fullscreen when double-clicking the video area itself
    if (el.closest(".controls-overlay") || el.closest("[data-panel]")) return;
    toggleFullscreen();
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="w-screen h-screen relative overflow-hidden"
  style="background: {player.duration > 0 ? 'transparent' : 'black'}; cursor: {!player.fullscreen || player.controlsVisible ? 'default' : 'none'};"
  onmousemove={handleMouseMove}
  ondblclick={handleDoubleClick}
  oncontextmenu={handleContextMenu}
>
  {#if dragOver}
    <div class="absolute inset-0 z-[90] flex items-center justify-center bg-black/60 border-2 border-dashed border-white/30 pointer-events-none">
      <p class="text-white/60 text-sm">{t().dropToPlay}</p>
    </div>
  {/if}

  {#if !fileLoaded && player.duration === 0}
    <div class="absolute inset-0 flex flex-col items-center justify-center text-white/40">
      <svg class="w-20 h-20 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1"
          d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1"
          d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <p class="text-sm mb-1">{t().welcomeTitle}</p>
      <p class="text-xs text-white/25">
        {t().welcomeSubtitle} <kbd class="px-1.5 py-0.5 bg-white/10 rounded text-white/40">Ctrl+O</kbd> {t().toOpen}
      </p>
    </div>
  {/if}

  <TitleBar visible={player.controlsVisible} menuOpen={ctxShow} onmenu={(mx, my) => { if (mx < 0) { ctxShow = false; } else { ctxX = mx; ctxY = my; ctxShow = true; } }} />
  <VideoControls visible={player.controlsVisible} bind:settingsOpen />

  <ContextMenu
    show={ctxShow}
    x={ctxX}
    y={ctxY}
    onclose={() => ctxShow = false}
    onopen={handleOpenFile}
    onpanel={openPanel}
  />

  <MediaInfoPanel bind:visible={infoPanel} />
</div>
