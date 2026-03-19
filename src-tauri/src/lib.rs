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
            commands::playback::screenshot,
            commands::playback::frame_step,
            commands::playback::frame_back_step,
            commands::playback::toggle_ab_loop,
            commands::playback::get_chapters,
            commands::playback::seek_chapter,
            commands::playback::open_url,
            // Tracks
            commands::tracks::get_tracks,
            commands::tracks::select_subtitle,
            commands::tracks::select_audio_track,
            commands::tracks::load_subtitle,
            commands::tracks::toggle_subtitles,
            commands::tracks::set_subtitle_delay,
            commands::tracks::set_audio_delay,
            commands::tracks::set_sub_style,
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
            // Video
            commands::video::set_brightness,
            commands::video::set_contrast,
            commands::video::set_saturation,
            commands::video::toggle_deinterlace,
            commands::video::set_video_zoom,
            commands::video::set_video_pan,
            commands::video::get_video_zoom_pan,
            commands::video::reset_video_zoom_pan,
            commands::video::set_aspect_ratio,
            commands::video::get_aspect_ratio,
            // Audio FX
            commands::audio_fx::set_audio_normalization,
            commands::audio_fx::set_audio_equalizer,
            commands::audio_fx::reset_audio_equalizer,
            // Media Info
            commands::media_info::get_media_info,
            // Translate
            commands::translate::translate_subtitles,
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
