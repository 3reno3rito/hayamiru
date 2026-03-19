use std::path::Path;
use std::sync::Arc;
use tauri::{Emitter, State};
use crate::error::AppError;
use crate::services::{subtitle_extract, tracks::TracksService, translate};
use crate::state::{AppState, MpvState};

#[derive(Clone, serde::Serialize)]
struct TranslateProgress { current: usize, total: usize, done: bool }

#[tauri::command]
pub async fn translate_subtitles(
    target_lang: String, app: tauri::AppHandle,
    mpv_state: State<'_, MpvState>, app_state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mpv = mpv_state.get()?;
    let video_path = app_state.with(|_, f| f.clone())?
        .ok_or_else(|| AppError::Config("No file playing".into()))?;

    // Find the selected subtitle track
    let tracks = TracksService::get_all(mpv);
    let sub_track = tracks.iter().find(|t| t.track_type == "sub" && t.selected)
        .ok_or_else(|| AppError::Config("No subtitle track selected".into()))?;

    // Determine SRT output path
    let srt_path = build_srt_path(&video_path, &target_lang);

    // Cached — just load
    if Path::new(&srt_path).exists() {
        mpv.command(&["sub-add", &srt_path, "auto"])?;
        let _ = app.emit("translate:progress", TranslateProgress { current: 1, total: 1, done: true });
        return Ok(srt_path);
    }

    let _ = app.emit("translate:progress", TranslateProgress { current: 0, total: 0, done: false });

    // Extract entries based on subtitle source
    let entries = if sub_track.external && !sub_track.external_filename.is_empty() {
        let ext_path = &sub_track.external_filename;
        let ext = Path::new(ext_path).extension()
            .and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        match ext.as_str() {
            "srt" => subtitle_extract::extract_from_srt(ext_path),
            "ass" | "ssa" => subtitle_extract::extract_from_ass(ext_path),
            _ => subtitle_extract::extract_from_srt(ext_path),
        }
    } else {
        // Embedded subtitle — extract via ffprobe/ffmpeg (exact timing)
        subtitle_extract::extract_from_video(&video_path, Some(sub_track.id))
    }.map_err(AppError::Config)?;

    if entries.is_empty() { return Err(AppError::Config("No subtitle entries found".into())); }

    // Chunk by ~4500 chars
    let mut chunks: Vec<Vec<usize>> = Vec::new();
    let (mut cur, mut len) = (Vec::new(), 0usize);
    for (i, e) in entries.iter().enumerate() {
        let l = e.text.len() + 2; // +2 for \n\n separator
        if len + l > 4500 && !cur.is_empty() { chunks.push(cur); cur = Vec::new(); len = 0; }
        cur.push(i); len += l;
    }
    if !cur.is_empty() { chunks.push(cur); }

    let total = chunks.len();
    let lang = Arc::new(target_lang);
    let entries_arc = Arc::new(entries.clone());

    // All chunks in parallel
    let mut handles = Vec::with_capacity(total);
    for (idx, indices) in chunks.into_iter().enumerate() {
        let (lang, entries_ref, app_c) = (lang.clone(), entries_arc.clone(), app.clone());
        handles.push(tokio::spawn(async move {
            let sep = "\n\n";
            let combined: String = indices.iter()
                .map(|&i| entries_ref[i].text.as_str())
                .collect::<Vec<_>>().join(sep);
            let result = translate::translate(&combined, &lang).await;
            let _ = app_c.emit("translate:progress", TranslateProgress { current: idx + 1, total, done: false });
            (indices, result)
        }));
    }

    let mut translated = entries;
    for h in handles {
        let (indices, result) = h.await.map_err(|e| AppError::Config(e.to_string()))?;
        if let Ok(t) = result {
            let parts: Vec<&str> = t.split("\n\n").collect();
            for (j, &idx) in indices.iter().enumerate() {
                if j < parts.len() && idx < translated.len() {
                    translated[idx].text = parts[j].trim().to_string();
                }
            }
        }
    }

    subtitle_extract::write_srt(&translated, &srt_path).map_err(AppError::Config)?;
    mpv.command(&["sub-add", &srt_path, "auto"])?;
    let _ = app.emit("translate:progress", TranslateProgress { current: total, total, done: true });
    Ok(srt_path)
}

fn build_srt_path(video_path: &str, lang: &str) -> String {
    let p = Path::new(video_path);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("sub");
    let parent = p.parent().and_then(|p| p.to_str()).unwrap_or(".");
    format!("{parent}/{stem}.{lang}.srt")
}
