//! Agent store — pure CRUD for in-memory sub-agent registry.

use crate::session::Session;
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
pub(super) struct AgentEntry {
    pub instructions: String,
    pub session: Session,
}

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

pub(super) fn insert(name: String, entry: AgentEntry) {
    AGENT_STORE.write().insert(name, entry);
}

pub(super) fn remove(name: &str) -> Option<AgentEntry> {
    AGENT_STORE.write().remove(name)
}

pub(super) fn get(name: &str) -> Option<AgentEntry> {
    AGENT_STORE.read().get(name).cloned()
}

pub(super) fn contains(name: &str) -> bool {
    AGENT_STORE.read().contains_key(name)
}

pub(super) fn list() -> Vec<(String, String, usize)> {
    AGENT_STORE
        .read()
        .iter()
        .map(|(n, e)| (n.clone(), e.instructions.clone(), e.session.messages.len()))
        .collect()
}

pub(super) fn update_session(name: &str, session: Session) {
    if let Some(e) = AGENT_STORE.write().get_mut(name) {
        e.session = session;
    }
}

pub(super) fn snapshots() -> Vec<super::AgentSnapshot> {
    AGENT_STORE
        .read()
        .iter()
        .map(|(n, e)| super::AgentSnapshot {
            name: n.clone(),
            instructions: e.instructions.clone(),
            session: e.session.clone(),
        })
        .collect()
}
