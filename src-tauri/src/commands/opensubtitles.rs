use std::path::Path;
use tauri::State;
use crate::error::AppError;
use crate::services::opensubtitles::{self, SubResult};
use crate::state::{AppState, MpvState};

#[tauri::command]
pub async fn search_subtitles(
    lang: String,
    app_state: State<'_, AppState>,
) -> Result<Vec<SubResult>, AppError> {
    let (api_key, username, password) = app_state.with(|s, _| {
        (s.os_api_key.clone(), s.os_username.clone(), s.os_password.clone())
    })?;
    if api_key.is_empty() { return Err(AppError::Config("OpenSubtitles API key not set".into())); }

    let video_path = app_state.with(|_, f| f.clone())?
        .ok_or_else(|| AppError::Config("No file playing".into()))?;

    // Init/reuse shared client (login happens once)
    opensubtitles::get_client(&api_key, &username, &password).await.map_err(AppError::Config)?;

    // Compute hash in blocking context (file I/O)
    let path = video_path.clone();
    let hash = tokio::task::spawn_blocking(move || {
        opensubtitles::compute_hash(&path).unwrap_or_default()
    }).await.unwrap_or_default();

    let filename = Path::new(&video_path).file_stem()
        .and_then(|s| s.to_str()).unwrap_or("").to_string();

    opensubtitles::search(&hash, &filename, &lang).await.map_err(AppError::Config)
}

#[tauri::command]
pub async fn download_subtitle(
    file_id: i64,
    file_name: String,
    app_state: State<'_, AppState>,
    mpv_state: State<'_, MpvState>,
) -> Result<String, AppError> {
    let content = opensubtitles::download(file_id).await.map_err(AppError::Config)?;

    let video_path = app_state.with(|_, f| f.clone())?
        .ok_or_else(|| AppError::Config("No file playing".into()))?;

    // Sanitize filename — remove path traversal
    let safe_name = Path::new(&file_name).file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("subtitle.srt");
    let sub_path = Path::new(&video_path).parent().unwrap_or(Path::new(".")).join(safe_name);
    let sub_str = sub_path.to_string_lossy().to_string();

    std::fs::write(&sub_path, &content).map_err(|e| AppError::Config(format!("Write failed: {e}")))?;

    let mpv = mpv_state.get()?;
    mpv.command(&["sub-add", &sub_str, "select"])?;

    Ok(sub_str)
}
