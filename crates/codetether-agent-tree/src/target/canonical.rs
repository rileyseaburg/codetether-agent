//! Canonical path projection for a root-scoped target.

use crate::{Entry, Host};
use anyhow::Result;
use std::collections::HashMap;

/// Resolve a target and project its canonical path from a fresh registry snapshot.
/// # Arguments
/// * `current` — Calling session identity.
/// * `target` — Child ID, relative path or absolute root path.
/// # Returns
/// The canonical child path, or none when no scoped target remains.
/// # Errors
/// Returns target-prefix resolution errors.
/// # Examples
/// This uses [crate::resolve] before taking the path-projection snapshot.
pub fn get<H: Host>(current: &str, target: &str) -> Result<Option<String>> {
    let Some(id) = super::resolve::<H>(current, target)? else {
        return Ok(None);
    };
    let entries = H::entries()
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, H::Entry>>();
    let root = super::super::path::root(current, &entries);
    Ok(super::super::path::for_session(&id, &root, &entries))
}
