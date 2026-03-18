<script lang="ts">
  import { getMediaInfo, type MediaInfo } from "$lib/bindings/media-info";
  import { t } from "$lib/i18n/index.svelte";

  let { visible = $bindable(false) }: { visible: boolean } = $props();

  let info = $state<MediaInfo | null>(null);

  $effect(() => {
    if (visible) getMediaInfo().then((i) => { info = i; }).catch(() => {});
  });

  function fmt(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
    return `${(bytes / 1073741824).toFixed(2)} GB`;
  }

  function fmtBitrate(bps: number): string {
    if (bps < 1000) return `${bps} bps`;
    if (bps < 1000000) return `${(bps / 1000).toFixed(0)} kbps`;
    return `${(bps / 1000000).toFixed(1)} Mbps`;
  }

  function fmtDuration(s: number): string {
    const h = Math.floor(s / 3600);
    const m = Math.floor((s % 3600) / 60);
    const sec = Math.floor(s % 60);
    return h > 0 ? `${h}h ${m}m ${sec}s` : `${m}m ${sec}s`;
  }
</script>

{#if visible && info}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="fixed inset-0 z-[80]" onclick={() => visible = false}></div>

  <div data-panel class="fixed right-4 bottom-16 z-[81] w-[300px] bg-[#18181c]/95 backdrop-blur-md border border-white/10 rounded-xl shadow-2xl text-[13px] text-white/90 select-none">
    <div class="flex items-center border-b border-white/[0.08] px-3 py-2">
      <span class="font-medium text-xs">{t().mediaInfo}</span>
      <div class="flex-1"></div>
      <button class="ctrl-btn w-6 h-6 text-xs" onclick={() => visible = false}>✕</button>
    </div>

    <div class="px-3 py-2 space-y-1.5">
      <div class="flex justify-between"><span class="text-white/50">File</span><span class="text-right truncate ml-4 max-w-[180px]">{info.filename}</span></div>
      {#if info.width > 0}
        <div class="flex justify-between"><span class="text-white/50">Resolution</span><span>{info.width}×{info.height}</span></div>
      {/if}
      {#if info.video_codec}
        <div class="flex justify-between"><span class="text-white/50">Video Codec</span><span>{info.video_codec}</span></div>
      {/if}
      {#if info.audio_codec}
        <div class="flex justify-between"><span class="text-white/50">Audio Codec</span><span>{info.audio_codec}</span></div>
      {/if}
      {#if info.fps > 0}
        <div class="flex justify-between"><span class="text-white/50">FPS</span><span>{info.fps.toFixed(2)}</span></div>
      {/if}
      {#if info.video_bitrate > 0}
        <div class="flex justify-between"><span class="text-white/50">Video Bitrate</span><span>{fmtBitrate(info.video_bitrate)}</span></div>
      {/if}
      {#if info.audio_bitrate > 0}
        <div class="flex justify-between"><span class="text-white/50">Audio Bitrate</span><span>{fmtBitrate(info.audio_bitrate)}</span></div>
      {/if}
      <div class="flex justify-between"><span class="text-white/50">Duration</span><span>{fmtDuration(info.duration)}</span></div>
      {#if info.file_size > 0}
        <div class="flex justify-between"><span class="text-white/50">File Size</span><span>{fmt(info.file_size)}</span></div>
      {/if}
    </div>
  </div>
{/if}
