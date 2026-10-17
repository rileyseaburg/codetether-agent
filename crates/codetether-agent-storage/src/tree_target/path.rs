//! Root discovery and canonical paths for durable manifests.

use super::Manifest;
use std::collections::{HashMap, HashSet};

pub(super) fn root(current: &str, entries: &HashMap<&str, &Manifest>) -> String {
    let mut cursor = current.to_string();
    let mut seen = HashSet::new();
    while seen.insert(cursor.clone()) {
        let Some(parent) = entries
            .get(cursor.as_str())
            .and_then(|entry| entry.owner_session_id.clone())
        else {
            break;
        };
        cursor = parent;
    }
    cursor
}

pub(super) fn canonical(
    id: &str,
    root: &str,
    entries: &HashMap<&str, &Manifest>,
) -> Option<String> {
    if id == root {
        return Some("/root".into());
    }
    let mut cursor = id;
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    while cursor != root && seen.insert(cursor) {
        let entry = entries.get(cursor)?;
        names.push(entry.name.as_str());
        cursor = entry.owner_session_id.as_deref()?;
    }
    (cursor == root).then(|| {
        names.reverse();
        format!("/root/{}", names.join("/"))
    })
}
