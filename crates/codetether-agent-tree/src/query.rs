//! Sorted root-tree query and lifecycle projection.

use crate::{Entry, Host};
use anyhow::{Context, Result};
use codetether_collaboration_state::ThreadStatus;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
/// One visible agent path and its current lifecycle status.
///
/// # Examples
/// ```
/// let agent=codetether_agent_tree::ListedAgent {agent_name:"/root".into(),agent_status:codetether_collaboration_state::ThreadStatus::Running};
/// assert_eq!(agent.agent_name,"/root");
/// ```
pub struct ListedAgent {
    /// Canonical path within the caller root tree.
    pub agent_name: String,
    /// Current status or the existing execution-state fallback.
    pub agent_status: ThreadStatus,
}

/// List the current root tree, optionally filtered by a relative or absolute prefix.
/// # Arguments
/// * `current` — Calling session identity.
/// * `path_prefix` — Optional segment-aware path prefix.
/// # Returns
/// Sorted agent paths and current lifecycle statuses.
/// # Errors
/// Returns detached-root or invalid-prefix errors.
/// # Examples
/// Implement [crate::Host] for the live registry before listing.
pub fn list<H: Host>(current: &str, path_prefix: Option<&str>) -> Result<Vec<ListedAgent>> {
    let entries = H::entries()
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, H::Entry>>();
    let root_id = super::path::root(current, &entries);
    let current_path = super::path::for_session(current, &root_id, &entries)
        .context("current agent is detached from its root tree")?;
    let prefix = super::prefix::resolve(&current_path, path_prefix)?;
    let mut agents = Vec::new();
    if super::prefix::matches("/root", prefix.as_deref()) {
        agents.push(ListedAgent {
            agent_name: "/root".into(),
            agent_status: ThreadStatus::Running,
        });
    }
    for (id, entry) in &entries {
        let Some(path) = super::path::for_session(id, &root_id, &entries) else {
            continue;
        };
        if super::prefix::matches(&path, prefix.as_deref()) {
            agents.push(ListedAgent {
                agent_name: path,
                agent_status: super::status::for_entry(entry),
            });
        }
    }
    agents.sort_by(|left, right| left.agent_name.cmp(&right.agent_name));
    Ok(agents)
}
