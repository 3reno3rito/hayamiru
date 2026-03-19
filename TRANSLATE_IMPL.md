# Subtitle Translation — Implementation Guide

## What it does
Extracts subtitles from MKV (embedded ASS/SRT) → batch translates via Google free API → generates `.{lang}.srt` file → loads in mpv as new track. Sub-second for a full movie.

## Architecture (3 files backend, 2 files frontend)

```
Backend (Rust):
  services/translate.rs          — Google API client (POST, ~5000 chars/request, HashMap cache)
  services/subtitle_extract.rs   — MKV demuxer + ASS/SRT parser + SRT writer
  commands/translate.rs          — Tauri command: orchestrates extract → translate → write → load

Frontend (Svelte):
  bindings/translate.ts          — invoke("translate_subtitles")
  SubtitlePanel.svelte           — Translate button + language selector + progress bar
```

## Backend — 3 files

### 1. `services/translate.rs` (~60 lines)

Single responsibility: translate text via Google free API.

```rust
// POST to https://translate.googleapis.com/translate_a/single
// Params: client=gtx, sl=auto, tl={target}, dt=t
// Body form: q={text}
// Response: [[["translated","original",...],...],...] → concatenate [0][0] of each segment
```

Key decisions:
- **POST not GET** — 5000 chars vs 1800 chars per request
- **No line splitting** — send the entire chunk as-is with \n separators, Google handles it
- **HashMap cache** — keyed by (text, lang), avoids re-translating on re-run
- **No retry logic here** — keep it simple, caller handles errors

Dependencies: `reqwest` (with `json` feature), `urlencoding`, `serde_json`

### 2. `services/subtitle_extract.rs` (~180 lines)

Two jobs: extract entries from video, write SRT output.

**Extract from MKV:**
```rust
MatroskaFile::open(file)
→ find first text subtitle track (skip PGS/VobSub bitmap tracks)
→ iterate frames with next_frame()
→ parse ASS text: splitn(9, ',') → parts[8] = text (strip ASS tags)
→ Vec<SubEntry { start_ms, end_ms, text }>
```

**ASS codec detection** — skip bitmap codecs:
```rust
fn is_bitmap_codec(id: &str) -> bool {
    matches!(id, "S_HDMV/PGS" | "S_VOBSUB" | "S_DVBSUB")
}
```

**ASS text parsing** — MKV stores ASS events as:
```
ReadOrder,Layer,Style,Name,MarginL,MarginR,MarginV,Effect,Text
```
9 fields (8 commas). Text is index 8. Use `splitn(9, ',')`.
Then strip `{...}` tags and replace `\N` with `\n`.

**Write SRT:**
```
1
00:01:23,456 --> 00:01:25,789
Translated text here

2
...
```

Dependencies: `matroska-demuxer`

### 3. `commands/translate.rs` (~120 lines)

Orchestration command called from frontend:

```
1. Get current video path from AppState
2. Check if .{lang}.srt already exists → if yes, sub-add and return
3. Extract subtitle entries from MKV
4. Build chunks by CHARACTER LIMIT (4500 chars each, not line count)
5. Fire ALL chunks in parallel (tokio::spawn)
6. Collect results, map back to entries
7. Write SRT file next to video
8. mpv.command(["sub-add", path, "auto"])
9. Emit progress events to frontend
```

**Chunking by chars, not lines:**
```rust
let max_chars = 4500;
// accumulate lines until ~4500 chars, then start new chunk
// join lines with \n → Google translates as one block
// split response on \n → map back to entries
```

**Why 4500 and not 5000:** leave margin for URL encoding expansion.

**Parallel execution:**
```rust
for chunk in chunks {
    handles.push(tokio::spawn(async move {
        translate::translate(&combined, &lang).await
    }));
}
// ~9 chunks for a full movie, all in parallel = ~200-500ms
```

**SRT path:** `{video_dir}/{video_stem}.{lang}.srt`

Dependencies: `tokio` (time feature for sleep on retry)

## Frontend — 2 changes

### 1. `bindings/translate.ts` (3 lines)
```ts
export const translateSubtitles = (targetLang: string) =>
  invoke<string>("translate_subtitles", { targetLang });
```

### 2. `SubtitlePanel.svelte` additions (~40 lines)

Add to existing panel:
- Language selector (Select component with lang codes)
- "Translate" button
- Progress bar (listen to `translate:progress` event)
- Error display

```svelte
// State
let translateLang = $state("pt");
let translating = $state(false);
let translateProgress = $state(0);
let translateTotal = $state(0);

// Handler
async function handleTranslate() {
  translating = true;
  const unlisten = await listen("translate:progress", (e) => {
    translateProgress = e.payload.current;
    translateTotal = e.payload.total;
    if (e.payload.done) { translating = false; refresh(); }
  });
  await translateSubtitles(translateLang);
  unlisten();
}
```

## Cargo.toml additions
```toml
reqwest = { version = "0.12", features = ["json"] }
urlencoding = "2"
matroska-demuxer = "0.4"
tokio = { version = "1", features = ["time"] }
```

## Register in lib.rs
```rust
commands::translate::translate_subtitles,
```

## Register modules
```rust
// services/mod.rs
pub mod subtitle_extract;
pub mod translate;

// commands/mod.rs
pub mod translate;
```

## Performance numbers
| Step | Time |
|------|------|
| MKV parsing (1500 entries) | <100ms |
| Chunking + joining | <1ms |
| Google API (9 parallel POST requests) | ~200-500ms |
| SRT writing | <10ms |
| mpv sub-add | <10ms |
| **Total** | **~300-600ms** |

## What NOT to do
- Don't translate line by line (1500 requests = 30+ seconds)
- Don't use GET requests (1800 char limit vs 5000 with POST)
- Don't split text by \n before sending to Google (defeats batching)
- Don't use sequential requests (use parallel tokio::spawn)
- Don't use ffmpeg for extraction (matroska-demuxer is faster, no external dependency)
- Don't use real-time translation overlay (timing delay, polling overhead)

## Edge cases handled
- Bitmap subtitles (PGS/VobSub) → skipped, only text tracks extracted
- ASS metadata in text → stripped via splitn(9, ',')[8]
- ASS style tags `{...}` → stripped
- `\N` line breaks in ASS → converted to `\n`
- Existing translation → loads cached .srt instantly, no API calls
- Rate limit (429/403) → error propagated to UI
- No subtitle track → error message in UI
- Text with commas → splitn(9) preserves commas in subtitle text field
