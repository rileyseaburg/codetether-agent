//! Mutable state owned by one workspace mux server.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::MuxSession;
use crate::mux::isolation::Isolation;

/// Serializable state for one server: one checkout, many isolated sessions.
///
/// `sessions` never share windows or runtime state. The checkout under
/// `workspace` is shared, so lease coordination lives at the server level.
/// Deserialization also accepts the legacy single-session shape.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(in crate::mux) struct MuxSnapshot {
    /// Canonical checkout served by this process.
    pub workspace: PathBuf,
    /// Workspace allocation policy applied to new windows.
    pub isolation: Isolation,
    pub sessions: Vec<MuxSession>,
}

/// Derived mirror of the current wire shape used by the tolerant decoder.
#[derive(Deserialize)]
pub(super) struct Current {
    workspace: PathBuf,
    #[serde(default)]
    isolation: Isolation,
    sessions: Vec<MuxSession>,
}

impl From<Current> for MuxSnapshot {
    fn from(current: Current) -> Self {
        Self {
            workspace: current.workspace,
            isolation: current.isolation,
            sessions: current.sessions,
        }
    }
}

impl MuxSnapshot {
    pub(in crate::mux) fn new(name: String, workspace: PathBuf, isolation: Isolation) -> Self {
        Self {
            sessions: vec![MuxSession::new(name, workspace.clone())],
            workspace,
            isolation,
        }
    }

    pub(in crate::mux) fn session(&self, name: &str) -> Option<&MuxSession> {
        self.sessions.iter().find(|session| session.name == name)
    }

    pub(in crate::mux) fn session_mut(&mut self, name: &str) -> Option<&mut MuxSession> {
        self.sessions
            .iter_mut()
            .find(|session| session.name == name)
    }

    /// Slot for the next window across every session; PTY ids are server-wide.
    pub(in crate::mux) fn next_window_id(&self) -> u64 {
        self.sessions
            .iter()
            .flat_map(|session| session.windows.iter().map(|window| window.id))
            .max()
            .map_or(0, |id| id + 1)
    }
}
