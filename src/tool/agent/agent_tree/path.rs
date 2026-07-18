//! Root discovery and canonical path construction.

use crate::tool::agent::store::AgentEntry;
use std::collections::{HashMap, HashSet};

pub(super) fn root(current: &str, entries: &HashMap<String, AgentEntry>) -> String {
    let mut cursor = current.to_string();
    let mut seen = HashSet::new();
    while seen.insert(cursor.clone()) {
        let Some(parent) = entries
            .get(&cursor)
            .and_then(|entry| entry.owner_session_id.clone())
        else {
            break;
        };
        cursor = parent;
    }
    cursor
}

pub(super) fn for_session(
    session_id: &str,
    root_id: &str,
    entries: &HashMap<String, AgentEntry>,
) -> Option<String> {
    if session_id == root_id {
        return Some("/root".into());
    }
    let mut cursor = session_id;
    let mut segments = Vec::new();
    let mut seen = HashSet::new();
    while cursor != root_id && seen.insert(cursor.to_string()) {
        let entry = entries.get(cursor)?;
        segments.push(entry.name.clone());
        cursor = entry.owner_session_id.as_deref()?;
    }
    (cursor == root_id).then(|| {
        segments.reverse();
        format!("/root/{}", segments.join("/"))
    })
}
