//! Bounded-memory session loading through the indexed store.
//!
//! `Session::load_tail` restores only the requested message and tool-use
//! windows. Legacy JSON is imported through the protected store migration path.

use std::path::PathBuf;

use anyhow::Result;

use super::types::Session;
/// Result of loading a bounded native session window.
pub type TailLoad = codetether_session_store::TailLoad<Session>;

impl Session {
    /// Load a session by UUID, keeping only the last `window` messages and
    /// tool uses. Use `window = 0` to drop all messages (cheap directory
    /// scan).
    ///
    /// # Errors
    /// Returns an error if the file cannot be opened or the JSON is malformed.
    pub async fn load_tail(id: &str, window: usize) -> Result<TailLoad> {
        codetether_session_store::lifecycle::load_tail::<Self>(id, window).await
    }

    /// Load a session from an explicit path with a bounded message window.
    pub async fn load_tail_from_path(path: PathBuf, window: usize) -> Result<TailLoad> {
        super::store::load(&path, window).await
    }
}
