//! Attached-client tracking of the latest server snapshot for one session.

use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::mux::model::{MuxSession, MuxSnapshot};
use crate::mux::protocol::ServerResponse;

/// What an attached client knows: the server snapshot and its own session.
pub(super) struct ClientState {
    pub session: String,
    pub snapshot: Option<MuxSnapshot>,
}

impl ClientState {
    pub(super) fn new(session: &str) -> Self {
        Self {
            session: session.to_string(),
            snapshot: None,
        }
    }

    pub(super) fn update(&mut self, response: &ServerResponse) {
        if let ServerResponse::Snapshot { state } = response {
            self.snapshot = Some(state.clone());
        }
    }

    pub(super) fn current(&self) -> Result<&MuxSession> {
        self.snapshot
            .as_ref()
            .and_then(|state| state.session(&self.session))
            .ok_or_else(|| anyhow!("mux session '{}' is unavailable", self.session))
    }

    pub(super) fn active_id(&self) -> Result<u64> {
        Ok(self.current()?.active_window)
    }

    pub(super) fn active_workspace(&self) -> Result<PathBuf> {
        self.current()?
            .active()
            .map(|window| window.workspace.clone())
            .ok_or_else(|| anyhow!("active mux window is unavailable"))
    }
}
