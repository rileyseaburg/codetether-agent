//! Root-scoped target resolution by ID, relative path, or absolute path.

use crate::tool::agent::store::{self, AgentEntry};
use anyhow::Result;
use std::collections::HashMap;

#[path = "target/canonical.rs"]
mod canonical_path;
pub(crate) use canonical_path::get as canonical;

pub(crate) fn resolve(current: &str, target: &str) -> Result<Option<String>> {
    let entries = store::entries_for_parent(None)
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, AgentEntry>>();
    let root_id = super::path::root(current, &entries);
    if entries.contains_key(target) {
        return Ok(super::path::for_session(target, &root_id, &entries).map(|_| target.to_string()));
    }
    let current_path = match super::path::for_session(current, &root_id, &entries) {
        Some(path) => path,
        None => return Ok(None),
    };
    let Some(expected) = super::prefix::resolve(&current_path, Some(target))? else {
        return Ok(None);
    };
    Ok(entries.iter().find_map(|(id, _)| {
        (super::path::for_session(id, &root_id, &entries).as_deref() == Some(&expected))
            .then(|| id.clone())
    }))
}
