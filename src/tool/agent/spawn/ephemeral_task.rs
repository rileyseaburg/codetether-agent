//! Workspace and task intent for one ephemeral agent invocation.

use crate::tool::agent::spawn::workspace::Handoff;
use crate::tool::agent::spawn_request::SpawnRequest;
use anyhow::Result;
use std::path::PathBuf;

pub(super) async fn policy(
    request: &SpawnRequest<'_>,
) -> Result<(PathBuf, bool, bool, Option<Handoff>)> {
    let workspace = request
        .parent_workspace
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| ".".into());
    let read_only = crate::tool::swarm_execute::support::is_read_only(
        request.name,
        request.instructions,
        None,
        None,
    );
    let expects_changes = crate::tool::swarm_execute::support::expects_changes(
        request.name,
        request.instructions,
        None,
        None,
    );
    let handoff = if read_only {
        None
    } else {
        Some(crate::tool::agent::spawn::workspace::allocate(&workspace).await?)
    };
    let workspace = handoff
        .as_ref()
        .map_or(workspace, |value| value.workspace.clone());
    Ok((workspace, read_only, expects_changes, handoff))
}

#[cfg(test)]
#[path = "ephemeral_task_tests.rs"]
mod tests;
