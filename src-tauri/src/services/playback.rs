use serde::Serialize;

use crate::error::MpvError;
use crate::mpv::player::MpvPlayer;

/// Pure playback logic — no Tauri dependency, fully testable.
pub struct PlaybackService;

impl PlaybackService {
    pub fn load_file(mpv: &MpvPlayer, path: &str) -> Result<(), MpvError> {
        mpv.command(&["loadfile", path])
    }

    pub fn toggle_pause(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["cycle", "pause"])
    }

    pub fn play(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.set("pause", false)
    }

    pub fn pause(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.set("pause", true)
    }

    pub fn stop(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["stop"])
    }

    pub fn seek_relative(mpv: &MpvPlayer, seconds: f64) -> Result<(), MpvError> {
        mpv.command(&["seek", &seconds.to_string(), "relative"])
    }

    pub fn seek_absolute(mpv: &MpvPlayer, seconds: f64) -> Result<(), MpvError> {
        mpv.command(&["seek", &seconds.to_string(), "absolute"])
    }

    pub fn set_volume(mpv: &MpvPlayer, volume: f64) -> Result<(), MpvError> {
        mpv.set("volume", volume)
    }

    pub fn set_speed(mpv: &MpvPlayer, speed: f64) -> Result<(), MpvError> {
        mpv.set("speed", speed)
    }

    pub fn get_state(mpv: &MpvPlayer) -> PlaybackState {
        PlaybackState {
            time_pos: mpv.get::<f64>("time-pos").unwrap_or(0.0),
            duration: mpv.get::<f64>("duration").unwrap_or(0.0),
            paused: mpv.get::<bool>("pause").unwrap_or(true),
            title: mpv.get_property_string("media-title").unwrap_or_default(),
            volume: mpv.get::<f64>("volume").unwrap_or(100.0),
        }
    }
}

#[derive(Serialize)]
pub struct PlaybackState {
    pub time_pos: f64,
    pub duration: f64,
    pub paused: bool,
    pub title: String,
    pub volume: f64,
}
