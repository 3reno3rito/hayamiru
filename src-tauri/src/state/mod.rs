use std::sync::{Arc, OnceLock};

use crate::error::MpvError;
use crate::mpv::player::MpvPlayer;

/// Lock-free access to the mpv player instance.
/// Initialized once during `init_player`, then read without any mutex.
pub struct MpvState(OnceLock<Arc<MpvPlayer>>);

impl MpvState {
    pub fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Initialize the player. Fails if already initialized.
    pub fn init(&self, player: MpvPlayer) -> Result<(), MpvError> {
        self.0
            .set(Arc::new(player))
            .map_err(|_| MpvError::api(-1, "mpv already initialized"))
    }

    /// Get a reference to the player. Fails if not yet initialized.
    pub fn get(&self) -> Result<&Arc<MpvPlayer>, MpvError> {
        self.0.get().ok_or(MpvError::NotInitialized)
    }
}
