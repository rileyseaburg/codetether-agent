//! Sorted root-tree query and lifecycle projection.

use super::super::thread_status::ThreadStatus;
use crate::tool::agent::store::{self, AgentEntry};
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub(crate) struct ListedAgent {
    pub(crate) agent_name: String,
    pub(crate) agent_status: ThreadStatus,
}

pub(crate) fn list(current: &str, path_prefix: Option<&str>) -> Result<Vec<ListedAgent>> {
    let entries = store::entries_for_parent(None)
        .into_iter()
        .map(|entry| (entry.id().to_string(), entry))
        .collect::<HashMap<String, AgentEntry>>();
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
