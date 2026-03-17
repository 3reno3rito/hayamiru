use std::path::Path;

use serde::Serialize;

use crate::error::MpvError;
use crate::mpv::player::MpvPlayer;

pub const MEDIA_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "mpg", "mpeg", "m4v", "3gp", "ts", "vob",
    "mp3", "flac", "wav", "ogg", "m4a", "aac", "opus", "wma",
];

#[derive(Serialize)]
pub struct PlaylistItem {
    pub index: i64,
    pub filename: String,
    pub current: bool,
    pub title: String,
}

pub struct PlaylistService;

impl PlaylistService {
    /// Add a file or folder to the playlist.
    pub fn add(mpv: &MpvPlayer, path: &str) -> Result<(), MpvError> {
        if Path::new(path).is_dir() {
            let files = scan_media_folder(path);
            for file in &files {
                mpv.command(&["loadfile", file, "append-play"])?;
            }
            return Ok(());
        }
        mpv.command(&["loadfile", path, "append-play"])
    }

    pub fn remove(mpv: &MpvPlayer, index: i64) -> Result<(), MpvError> {
        mpv.command(&["playlist-remove", &index.to_string()])
    }

    pub fn next(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["playlist-next"])
    }

    pub fn prev(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["playlist-prev"])
    }

    pub fn play_index(mpv: &MpvPlayer, index: i64) -> Result<(), MpvError> {
        mpv.set::<&str>("playlist-pos", &index.to_string())
    }

    pub fn clear(mpv: &MpvPlayer) -> Result<(), MpvError> {
        mpv.command(&["playlist-clear"])
    }

    /// Open a file and populate playlist with all sibling media files.
    /// Optionally resumes from a saved position.
    pub fn open_with_siblings(
        mpv: &MpvPlayer,
        path: &str,
        resume_pos: Option<f64>,
    ) -> Result<(), MpvError> {
        let target = Path::new(path);
        let siblings = match target.parent() {
            Some(parent) => scan_media_folder(&parent.to_string_lossy()),
            None => vec![path.to_string()],
        };

        let target_str = target.to_string_lossy().to_string();
        let target_idx = siblings
            .iter()
            .position(|s| *s == target_str)
            .unwrap_or(0);

        // Order: target first, then remaining in natural order
        let mut ordered = Vec::with_capacity(siblings.len());
        ordered.extend_from_slice(&siblings[target_idx..]);
        ordered.extend_from_slice(&siblings[..target_idx]);

        // Load first file (with optional resume position)
        let first = &ordered[0];
        if let Some(pos) = resume_pos {
            mpv.command(&["loadfile", first, "replace", &format!("start={pos}")])?;
        } else {
            mpv.command(&["loadfile", first])?;
        }

        // Append the rest
        for file in &ordered[1..] {
            mpv.command(&["loadfile", file, "append"])?;
        }

        Ok(())
    }

    pub fn get_all(mpv: &MpvPlayer) -> Vec<PlaylistItem> {
        let count: i64 = mpv
            .get_property_string("playlist/count")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let current: i64 = mpv
            .get_property_string("playlist-pos")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(-1);

        (0..count)
            .filter_map(|i| {
                let filename = mpv
                    .get_property_string(&format!("playlist/{i}/filename"))
                    .ok()?;
                let title = Path::new(&filename)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| filename.clone());
                Some(PlaylistItem {
                    index: i,
                    filename,
                    current: i == current,
                    title,
                })
            })
            .collect()
    }
}

/// Scan a directory for media files, sorted with natural ordering.
/// Single source of truth — used by both `add` and `open_with_siblings`.
fn scan_media_folder(dir: &str) -> Vec<String> {
    let mut files: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let p = entry.ok()?.path();
            if !p.is_file() {
                return None;
            }
            let ext = p.extension()?.to_str()?.to_lowercase();
            if MEDIA_EXTENSIONS.contains(&ext.as_str()) {
                Some(p.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    files.sort_by(|a, b| natural_sort_key(a).cmp(&natural_sort_key(b)));
    files
}

/// Natural sort key: "Episode 2" < "Episode 10".
fn natural_sort_key(s: &str) -> Vec<(bool, String)> {
    let mut result = Vec::new();
    let mut chunk = String::new();
    let mut is_digit = false;

    for c in s.chars() {
        let d = c.is_ascii_digit();
        if d != is_digit && !chunk.is_empty() {
            if is_digit {
                result.push((true, format!("{:>020}", chunk)));
            } else {
                result.push((false, chunk.to_lowercase()));
            }
            chunk.clear();
        }
        is_digit = d;
        chunk.push(c);
    }
    if !chunk.is_empty() {
        if is_digit {
            result.push((true, format!("{:>020}", chunk)));
        } else {
            result.push((false, chunk.to_lowercase()));
        }
    }
    result
}
