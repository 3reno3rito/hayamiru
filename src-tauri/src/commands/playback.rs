use tauri::{Manager, State};

use crate::error::AppError;
use crate::mpv::events;
use crate::mpv::player::MpvPlayer;
use crate::mpv::types::*;
use crate::services::playback::{PlaybackService, PlaybackState};
use crate::state::MpvState;

#[tauri::command]
pub async fn init_player(
    app: tauri::AppHandle,
    mpv_state: State<'_, MpvState>,
) -> Result<(), AppError> {
    // Skip if already initialized (e.g. HMR re-render)
    if mpv_state.is_initialized() {
        return Ok(());
    }

    let window = app
        .get_webview_window("main")
        .ok_or_else(|| AppError::FileNotFound("main window not found".into()))?;

    #[cfg(target_os = "windows")]
    let hwnd_val = {
        let hwnd = window
            .hwnd()
            .map_err(|e| AppError::Config(format!("HWND: {e}")))?;
        hwnd.0 as i64
    };

    #[cfg(not(target_os = "windows"))]
    let hwnd_val: i64 = {
        let _ = window;
        return Err(AppError::Config("Only Windows is supported".into()));
    };

    let mpv = MpvPlayer::new(hwnd_val)?;

    // Observe properties for event-driven frontend updates
    mpv.observe_property("time-pos", 1, MPV_FORMAT_DOUBLE)?;
    mpv.observe_property("duration", 2, MPV_FORMAT_DOUBLE)?;
    mpv.observe_property("pause", 3, MPV_FORMAT_FLAG)?;
    mpv.observe_property("volume", 4, MPV_FORMAT_DOUBLE)?;
    mpv.observe_property("media-title", 5, MPV_FORMAT_STRING)?;

    mpv_state.init(mpv)?;

    // Start event loop with Arc from state
    let mpv_arc = mpv_state.get()?.clone();
    events::start_event_loop(mpv_arc, app.clone());

    Ok(())
}

#[tauri::command]
pub async fn open_file(path: String, state: State<'_, MpvState>) -> Result<(), AppError> {
    if !std::path::Path::new(&path).exists() {
        return Err(AppError::FileNotFound(path));
    }
    let mpv = state.get()?;
    PlaybackService::load_file(mpv, &path)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_pause(state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::toggle_pause(state.get()?)?;
    Ok(())
}

#[tauri::command]
pub async fn play(state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::play(state.get()?)?;
    Ok(())
}

#[tauri::command]
pub async fn pause(state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::pause(state.get()?)?;
    Ok(())
}

#[tauri::command]
pub async fn stop(state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::stop(state.get()?)?;
    Ok(())
}

#[tauri::command]
pub async fn seek_relative(seconds: f64, state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::seek_relative(state.get()?, seconds)?;
    Ok(())
}

#[tauri::command]
pub async fn seek_absolute(seconds: f64, state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::seek_absolute(state.get()?, seconds)?;
    Ok(())
}



#[tauri::command]
pub async fn set_volume(volume: f64, state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::set_volume(state.get()?, volume)?;
    Ok(())
}

#[tauri::command]
pub async fn set_speed(speed: f64, state: State<'_, MpvState>) -> Result<(), AppError> {
    PlaybackService::set_speed(state.get()?, speed)?;
    Ok(())
}

#[tauri::command]
pub async fn get_playback_state(state: State<'_, MpvState>) -> Result<PlaybackState, AppError> {
    let mpv = state.get()?;
    Ok(PlaybackService::get_state(mpv))
}
