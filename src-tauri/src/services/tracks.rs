use serde::Serialize;

use crate::error::MpvError;
use crate::mpv::player::MpvPlayer;

#[derive(Serialize)]
pub struct TrackInfo {
    pub id: i64,
    pub track_type: String,
    pub title: String,
    pub lang: String,
    pub selected: bool,
}

pub struct TracksService;

impl TracksService {
    pub fn get_all(mpv: &MpvPlayer) -> Vec<TrackInfo> {
        let count: i64 = mpv
            .get_property_string("track-list/count")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        (0..count)
            .filter_map(|i| {
                let p = format!("track-list/{i}");
                Some(TrackInfo {
                    id: mpv.get_property_string(&format!("{p}/id")).ok()?.parse().ok()?,
                    track_type: mpv.get_property_string(&format!("{p}/type")).ok()?,
                    title: mpv.get_property_string(&format!("{p}/title")).unwrap_or_default(),
                    lang: mpv.get_property_string(&format!("{p}/lang")).unwrap_or_default(),
                    selected: mpv.get_property_string(&format!("{p}/selected")).ok().as_deref() == Some("yes"),
                })
            })
            .collect()
    }

    pub fn select_subtitle(mpv: &MpvPlayer, id: i64) -> Result<(), MpvError> {
        mpv.set::<&str>("sid", &id.to_string())
    }

    pub fn select_audio(mpv: &MpvPlayer, id: i64) -> Result<(), MpvError> {
        mpv.set::<&str>("aid", &id.to_string())
    }

    pub fn load_subtitle(mpv: &MpvPlayer, path: &str) -> Result<(), MpvError> {
        mpv.command(&["sub-add", path])
    }

    pub fn toggle_subtitles(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["cycle", "sub-visibility"])
    }

    pub fn set_subtitle_delay(mpv: &MpvPlayer, seconds: f64) -> Result<(), MpvError> {
        mpv.set("sub-delay", seconds)
    }

    pub fn set_audio_delay(mpv: &MpvPlayer, seconds: f64) -> Result<(), MpvError> {
        mpv.set("audio-delay", seconds)
    }
}
