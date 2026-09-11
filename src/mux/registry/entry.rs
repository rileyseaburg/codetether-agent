//! Persistent connection information for one workspace mux server.

use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::mux::model::MuxSnapshot;

/// Private discovery record stored with owner-only permissions.
///
/// One record exists per server process; the sessions it hosts live in
/// `state.sessions`. Records written by pre-multi-session servers carried a
/// top-level `name`, which the tolerant decoder folds into `key` and a
/// single session so `mux list` and `kill-all` keep working across an upgrade.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(in crate::mux) struct MuxRecord {
    /// Stable registry key for this server (derived from the workspace).
    pub key: String,
    pub address: SocketAddr,
    pub token: String,
    pub pid: u32,
    pub started_at: DateTime<Utc>,
    pub state: MuxSnapshot,
}

impl MuxRecord {
    /// Names of every session this server currently hosts.
    pub(in crate::mux) fn session_names(&self) -> impl Iterator<Item = &str> {
        self.state
            .sessions
            .iter()
            .map(|session| session.name.as_str())
    }

    pub(in crate::mux) fn hosts(&self, session: &str) -> bool {
        self.state.session(session).is_some()
    }
}
