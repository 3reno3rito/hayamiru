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

    /// Initialize the player. No-op if already initialized.
    pub fn init(&self, player: MpvPlayer) -> Result<(), MpvError> {
        let _ = self.0.set(Arc::new(player));
        Ok(())
    }

    /// Returns true if the player is already initialized.
    pub fn is_initialized(&self) -> bool {
        self.0.get().is_some()
    }

    /// Get a reference to the player. Fails if not yet initialized.
    pub fn get(&self) -> Result<&Arc<MpvPlayer>, MpvError> {
        self.0.get().ok_or(MpvError::NotInitialized)
    }
}
