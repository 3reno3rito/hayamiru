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
        .invoke_handler(tauri::generate_handler![
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
            commands::window::toggle_fullscreen,
            commands::window::set_always_on_top,
            commands::window::minimize_window,
            commands::window::maximize_window,
            commands::window::close_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Hayamiru");
}
