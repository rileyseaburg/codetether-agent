//! Mutable state owned by one workspace mux server.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::MuxSession;
use crate::isolation::Isolation;

/// Serializable state for one server: one checkout, many isolated sessions.
///
/// `sessions` never share windows or runtime state. The checkout under
/// `workspace` is shared, so lease coordination lives at the server level.
/// Deserialization also accepts the legacy single-session shape.
/// # Examples
/// ```
/// let snapshot = codetether_mux_model::MuxSnapshot::new("work".into(), "/repo".into(), codetether_mux_model::isolation::Isolation::Shared);
/// assert_eq!(snapshot.sessions.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MuxSnapshot {
    /// Canonical checkout served by this process.
    pub workspace: PathBuf,
    /// Workspace allocation policy applied to new windows.
    pub isolation: Isolation,
    /// Isolated sessions hosted by the server.
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
#[path = "server_methods.rs"]
mod server_methods;
