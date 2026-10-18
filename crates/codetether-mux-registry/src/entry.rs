//! Persistent connection information for one workspace mux server.

use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use codetether_mux_model::MuxSnapshot;

/// Private discovery record stored with owner-only permissions.
///
/// One record exists per server process; the sessions it hosts live in
/// `state.sessions`. Records written by pre-multi-session servers carried a
/// top-level `name`, which the tolerant decoder folds into `key` and a
/// single session so `mux list` and `kill-all` keep working across an upgrade.
/// # Examples
/// ```
/// let record = codetether_mux_registry::MuxRecord {
///     key: "ws-example".into(), address: "127.0.0.1:9000".parse().unwrap(),
///     token: "example".into(), pid: 1, started_at: chrono::Utc::now(),
///     state: codetether_mux_model::MuxSnapshot::new("work".into(), "/repo".into(), codetether_mux_model::isolation::Isolation::Shared),
/// };
/// assert!(record.hosts("work"));
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MuxRecord {
    /// Stable registry key for this server (derived from the workspace).
    pub key: String,
    /// Server TCP endpoint.
    pub address: SocketAddr,
    /// Credential used to authenticate to this server.
    pub token: String,
    /// Server process ID.
    pub pid: u32,
    /// Server start timestamp.
    pub started_at: DateTime<Utc>,
    /// Latest persisted server snapshot.
    pub state: MuxSnapshot,
}

impl MuxRecord {
    /// Names of every session this server currently hosts.
    pub fn session_names(&self) -> impl Iterator<Item = &str> {
        self.state
            .sessions
            .iter()
            .map(|session| session.name.as_str())
    }

    /// Whether the snapshot contains the exact session name.
    pub fn hosts(&self, session: &str) -> bool {
        self.state.session(session).is_some()
    }
}
