//! Root-scoped target resolution by ID, relative path, or absolute path.

use crate::{Entry, Host};
use anyhow::Result;
use std::collections::HashMap;

#[path = "target/canonical.rs"]
mod canonical_path;
pub use canonical_path::get as canonical;

/// Resolve a durable ID or path within the caller's root tree.
/// # Arguments
/// * `current` — Calling session identity.
/// * `target` — Child ID, relative path or absolute root path.
/// # Returns
/// The matching child ID, or none when absent or outside this tree.
/// # Errors
/// Returns invalid-prefix errors.
/// # Examples
/// Implement [crate::Host] for registry snapshots before resolving targets.
pub fn resolve<H: Host>(current: &str, target: &str) -> Result<Option<String>> {
    let entries = H::entries()
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, H::Entry>>();
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
