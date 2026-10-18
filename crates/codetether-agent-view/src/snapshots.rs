//! Unified local-child and remote-peer snapshots for TUI consumers.

use crate::{AgentSnapshot, Host, local::local, remote::remote};

/// Project local children followed by observed peers in an optional owner scope.
/// # Arguments
/// * `parent_session_id` — Owner ID, or none for all observed owners.
/// # Returns
/// Combined snapshots in their existing source iteration order.
/// # Examples
/// Use an application [Host] implementation.
pub fn all<H: Host>(parent_session_id: Option<&str>) -> Vec<AgentSnapshot> {
    let mut agents = local::<H>(parent_session_id);
    agents.extend(remote(parent_session_id));
    agents
}

/// Look up an agent only when it belongs to the requested parent.
/// # Arguments
/// * `name` — Agent display name or canonical identity.
/// * `parent_session_id` — Required owner scope.
/// # Returns
/// The first matching local/peer snapshot, or none.
/// # Examples
/// Use an application [Host] implementation.
pub fn find<H: Host>(name: &str, parent_session_id: &str) -> Option<AgentSnapshot> {
    all::<H>(Some(parent_session_id))
        .into_iter()
        .find(|agent| agent.name == name || agent.id == name)
}
