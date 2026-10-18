//! Attached-client tracking of the latest server snapshot for one session.

use anyhow::{Result, anyhow};
use std::path::PathBuf;

use codetether_mux_model::{MuxSession, MuxSnapshot};
use codetether_mux_protocol::ServerResponse;

/// What an attached client knows: the server snapshot and its own session.
/// # Examples
/// ```
/// let state = codetether_mux_client::ClientState::new("work");
/// assert!(state.current().is_err());
/// ```
pub struct ClientState {
    /// Session selected by the client.
    pub session: String,
    /// Most recent server snapshot, if received.
    pub snapshot: Option<MuxSnapshot>,
}

impl ClientState {
    /// Create client state with no snapshot received yet.
    pub fn new(session: &str) -> Self {
        Self {
            session: session.to_string(),
            snapshot: None,
        }
    }

    /// Replace the cached snapshot only for snapshot responses.
    pub fn update(&mut self, response: &ServerResponse) {
        if let ServerResponse::Snapshot { state } = response {
            self.snapshot = Some(state.clone());
        }
    }

    /// Look up the selected session in the cached snapshot.
    /// # Errors
    /// Returns an error if no snapshot or selected session is available.
    pub fn current(&self) -> Result<&MuxSession> {
        self.snapshot
            .as_ref()
            .and_then(|state| state.session(&self.session))
            .ok_or_else(|| anyhow!("mux session '{}' is unavailable", self.session))
    }

    /// Return the selected session's active window ID.
    /// # Errors
    /// Propagates an unavailable-session error.
    pub fn active_id(&self) -> Result<u64> {
        Ok(self.current()?.active_window)
    }

    /// Return the selected session's active window workspace.
    /// # Errors
    /// Reports an unavailable session or missing active window.
    pub fn active_workspace(&self) -> Result<PathBuf> {
        self.current()?
            .active()
            .map(|window| window.workspace.clone())
            .ok_or_else(|| anyhow!("active mux window is unavailable"))
    }
}
