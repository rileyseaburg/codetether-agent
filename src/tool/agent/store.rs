//! Agent store — pure CRUD for in-memory sub-agent registry.
//!
//! This module owns the in-memory store used by the agent tool to track active
//! spawned sessions.
//!
//! # Examples
//!
//! ```ignore
//! let agents = list();
//! ```

use crate::session::Session;
use parking_lot::RwLock;
use std::collections::HashMap;

/// In-memory record for a spawned sub-agent.
///
/// # Examples
///
/// ```ignore
/// let entry = AgentEntry { instructions: "Review".into(), session, parent: None, depth: 0, model_id: None };
/// ```
#[derive(Clone)]
pub(super) struct AgentEntry {
    pub instructions: String,
    pub session: Session,
    pub parent: Option<String>,
    pub depth: u8,
    pub model_id: Option<String>,
}

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

/// Inserts or replaces a spawned agent entry.
pub(super) fn insert(name: String, entry: AgentEntry) {
    AGENT_STORE.write().insert(name, entry);
}

/// Removes a spawned agent entry by name.
pub(super) fn remove(name: &str) -> Option<AgentEntry> {
    AGENT_STORE.write().remove(name)
}

/// Returns a cloned spawned-agent entry when present.
pub(super) fn get(name: &str) -> Option<AgentEntry> {
    AGENT_STORE.read().get(name).cloned()
}

/// Returns whether an agent name is already present in the store.
pub(super) fn contains(name: &str) -> bool {
    AGENT_STORE.read().contains_key(name)
}

/// Lists spawned agents as `(name, instructions, message_count)` tuples.
pub(super) fn list() -> Vec<(String, String, usize)> {
    AGENT_STORE
        .read()
        .iter()
        .map(|(n, e)| (n.clone(), e.instructions.clone(), e.session.messages.len()))
        .collect()
}

/// Replaces the stored session for a spawned agent when it exists.
pub(super) fn update_session(name: &str, session: Session) {
    if let Some(e) = AGENT_STORE.write().get_mut(name) {
        e.session = session;
    }
}

/// Lists agents with full metadata for the TUI bridge (#297 Part A).
pub(super) fn list_with_metadata()
-> Vec<(String, String, usize, Option<String>, Option<String>, u8)> {
    AGENT_STORE
        .read()
        .iter()
        .map(|(n, e)| {
            (
                n.clone(),
                e.instructions.clone(),
                e.session.messages.len(),
                e.model_id.clone(),
                e.parent.clone(),
                e.depth,
            )
        })
        .collect()
}
