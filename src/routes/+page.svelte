<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { player } from "$lib/stores/player.svelte";
  import { initPlayer, openFile, togglePause, seekRelative, setVolume, getPlaybackState } from "$lib/bindings/playback";
  import { toggleFullscreen } from "$lib/bindings/window";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import VideoControls from "$lib/components/VideoControls.svelte";

  let controlsVisible = $state(true);
  let hideTimer: ReturnType<typeof setTimeout> | null = null;
  let fileLoaded = $state(false);
  let dragOver = $state(false);

  function showControls() {
    controlsVisible = true;
    if (hideTimer) clearTimeout(hideTimer);
    if (player.playing) {
      hideTimer = setTimeout(() => { controlsVisible = false; }, 3000);
    }
  }

  $effect(() => {
    const cleanups: Array<Promise<() => void> | (() => void)> = [];

    initPlayer()
      .then(() => {
        // Event-driven state updates (primary)
        cleanups.push(listen<number>("mpv:time-pos", (e) => { player.currentTime = e.payload; }));
        cleanups.push(listen<number>("mpv:duration", (e) => { player.duration = e.payload; }));
        cleanups.push(listen<boolean>("mpv:pause", (e) => { player.playing = !e.payload; }));
        cleanups.push(listen<number>("mpv:volume", (e) => { player.volume = e.payload; }));
        cleanups.push(listen<string>("mpv:media-title", (e) => { player.title = e.payload; }));
        cleanups.push(listen<void>("mpv:end-file", () => { player.playing = false; }));

        // Polling fallback — catches anything events miss
        const poll = setInterval(() => {
          getPlaybackState().then((s) => {
            player.currentTime = s.time_pos;
            player.duration = s.duration;
            player.playing = !s.paused;
            player.title = s.title;
            player.volume = s.volume;
          }).catch(() => {});
        }, 1000);
        cleanups.push(() => clearInterval(poll));
      })
      .catch((err) => console.error("Failed to init mpv:", err));

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

  function handleKeyDown(e: KeyboardEvent) {
    if (e.target instanceof HTMLInputElement) return;

    switch (e.key) {
      case " ": e.preventDefault(); togglePause(); break;
      case "f": case "F": case "F11": e.preventDefault(); toggleFullscreen(); break;
      case "ArrowRight": seekRelative(e.shiftKey ? 30 : 5); break;
      case "ArrowLeft": seekRelative(e.shiftKey ? -30 : -5); break;
      case "ArrowUp": e.preventDefault(); player.volume = Math.min(100, player.volume + 5); setVolume(player.volume); break;
      case "ArrowDown": e.preventDefault(); player.volume = Math.max(0, player.volume - 5); setVolume(player.volume); break;
      case "m": case "M": player.muted = !player.muted; setVolume(player.muted ? 0 : player.volume); break;
      case "Escape": if (player.fullscreen) toggleFullscreen(); break;
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
        { name: "Video", extensions: ["mp4","mkv","avi","mov","wmv","flv","webm","mpg","mpeg","m4v","ts","vob"] },
        { name: "Audio", extensions: ["mp3","flac","wav","ogg","m4a","aac","opus","wma"] },
        { name: "All", extensions: ["*"] },
      ],
    });
    if (selected) {
      fileLoaded = true;
      openFile(selected as string);
    }
  }

  function handleDoubleClick(e: MouseEvent) {
    if ((e.target as HTMLElement).closest(".controls-overlay")) return;
    toggleFullscreen();
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="w-screen h-screen relative overflow-hidden"
  style="background: {player.duration > 0 ? 'transparent' : 'black'}; cursor: {controlsVisible ? 'default' : 'none'};"
  onmousemove={showControls}
  ondblclick={handleDoubleClick}
>
  {#if dragOver}
    <div class="absolute inset-0 z-[90] flex items-center justify-center bg-black/60 border-2 border-dashed border-white/30 pointer-events-none">
      <p class="text-white/60 text-lg">Drop file to play</p>
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
      <p class="text-sm mb-1">Hayamiru (速観)</p>
      <p class="text-xs text-white/25">
        Drop a file here or press <kbd class="px-1.5 py-0.5 bg-white/10 rounded text-white/40">Ctrl+O</kbd> to open
      </p>
    </div>
  {/if}

  <TitleBar visible={controlsVisible} onopen={handleOpenFile} />
  <VideoControls visible={controlsVisible} />
</div>
