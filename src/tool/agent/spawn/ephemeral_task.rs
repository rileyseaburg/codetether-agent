//! Workspace and task intent for one ephemeral agent invocation.

use crate::tool::agent::spawn_request::SpawnRequest;
use std::path::PathBuf;

pub(super) fn policy(request: &SpawnRequest<'_>) -> (PathBuf, bool, bool) {
    let workspace = request
        .parent_workspace
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| ".".into());
    let read_only =
        crate::tool::swarm_execute::support::is_read_only(request.name, request.instructions, None);
    let expects_changes = crate::tool::swarm_execute::support::expects_changes(
        request.name,
        request.instructions,
        None,
    );
    (workspace, read_only, expects_changes)
}
