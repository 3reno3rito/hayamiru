mod commands;
mod error;
mod mpv;
mod services;
mod state;

use tracing_subscriber::EnvFilter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("hayamiru=info")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(state::MpvState::new())
        .manage(state::AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Playback
            commands::playback::init_player,
            commands::playback::open_file,
            commands::playback::toggle_pause,
            commands::playback::play,
            commands::playback::pause,
            commands::playback::stop,
            commands::playback::seek_relative,
            commands::playback::seek_absolute,
            commands::playback::set_volume,
            commands::playback::set_speed,
            commands::playback::get_playback_state,
            // Tracks
            commands::tracks::get_tracks,
            commands::tracks::select_subtitle,
            commands::tracks::select_audio_track,
            commands::tracks::load_subtitle,
            commands::tracks::toggle_subtitles,
            commands::tracks::set_subtitle_delay,
            commands::tracks::set_audio_delay,
            // Playlist
            commands::playlist::playlist_add,
            commands::playlist::playlist_remove,
            commands::playlist::playlist_next,
            commands::playlist::playlist_prev,
            commands::playlist::playlist_play_index,
            commands::playlist::playlist_clear,
            commands::playlist::get_playlist,
            // Settings
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::settings::get_recent_files,
            commands::settings::clear_recent_files,
            // Window
            commands::window::toggle_fullscreen,
            commands::window::set_always_on_top,
            commands::window::minimize_window,
            commands::window::maximize_window,
            commands::window::close_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Hayamiru");
}
