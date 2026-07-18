//! Canonical path projection for a root-scoped target.

use crate::tool::agent::store::{self, AgentEntry};
use anyhow::Result;
use std::collections::HashMap;

pub(crate) fn get(current: &str, target: &str) -> Result<Option<String>> {
    let Some(id) = super::resolve(current, target)? else {
        return Ok(None);
    };
    let entries = store::entries_for_parent(None)
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, AgentEntry>>();
    let root = super::super::path::root(current, &entries);
    Ok(super::super::path::for_session(&id, &root, &entries))
}