use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

use tauri::Emitter;
use tracing::{debug, info, warn};

use super::player::MpvPlayer;
use super::types::*;
use crate::state::take_pending_resume;

/// Spawn a named background thread that polls mpv events and emits them
/// as typed Tauri events to the frontend.
pub fn start_event_loop(mpv: Arc<MpvPlayer>, app: tauri::AppHandle) {
    std::thread::Builder::new()
        .name("mpv-events".into())
        .spawn(move || {
            info!("Event loop started");
            run_loop(&mpv, &app);
            info!("Event loop ended");
        })
        .expect("Failed to spawn mpv event loop thread");
}

fn run_loop(mpv: &MpvPlayer, app: &tauri::AppHandle) {
    loop {
        let evt = mpv.wait_event(0.5);

        match evt.event_id {
            MPV_EVENT_PROPERTY_CHANGE => {
                if evt.data.is_null() {
                    continue;
                }
                if let Err(e) = handle_property_change(evt, app) {
                    warn!(error = %e, "Failed to handle property change");
                }
            }

            MPV_EVENT_FILE_LOADED => {
                debug!("File loaded");
                if let Some(pos) = take_pending_resume() {
                    let _ = mpv.command(&["seek", &pos.to_string(), "absolute"]);
                }
                let _ = app.emit("mpv:file-loaded", ());
            }

            MPV_EVENT_END_FILE => {
                debug!("End of file");
                let _ = app.emit("mpv:end-file", ());
            }

            MPV_EVENT_SHUTDOWN => {
                info!("mpv shutdown event");
                break;
            }

            _ => {}
        }
    }
}

fn handle_property_change(evt: &MpvEvent, app: &tauri::AppHandle) -> Result<(), String> {
    let prop = unsafe { &*(evt.data as *const MpvEventProperty) };

    if prop.name.is_null() || prop.data.is_null() {
        return Ok(()); // Property value unavailable (e.g. during init)
    }

    let name = unsafe {
        CStr::from_ptr(prop.name)
            .to_str()
            .map_err(|e| e.to_string())?
    };

    unsafe {
        match (name, prop.format) {
            ("time-pos", MPV_FORMAT_DOUBLE) => {
                let val = *(prop.data as *const f64);
                let _ = app.emit("mpv:time-pos", val);
            }
            ("duration", MPV_FORMAT_DOUBLE) => {
                let val = *(prop.data as *const f64);
                let _ = app.emit("mpv:duration", val);
            }
            ("pause", MPV_FORMAT_FLAG) => {
                let val = *(prop.data as *const i32) != 0;
                let _ = app.emit("mpv:pause", val);
            }
            ("volume", MPV_FORMAT_DOUBLE) => {
                let val = *(prop.data as *const f64);
                let _ = app.emit("mpv:volume", val);
            }
            ("media-title", MPV_FORMAT_STRING) => {
                let ptr = *(prop.data as *const *const c_char);
                if !ptr.is_null() {
                    if let Ok(s) = CStr::from_ptr(ptr).to_str() {
                        let _ = app.emit("mpv:media-title", s.to_owned());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
