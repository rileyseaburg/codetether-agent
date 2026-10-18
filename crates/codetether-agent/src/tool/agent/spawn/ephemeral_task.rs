//! Test binding for the shared one-shot workspace policy.
use super::workspace::Handoff;
use crate::tool::agent::spawn_request::SpawnRequest;
use std::path::PathBuf;
pub(super) async fn policy(
    request: &SpawnRequest<'_>,
) -> anyhow::Result<(PathBuf, bool, bool, Option<Handoff>)> {
    codetether_agent_spawn::workspace_policy::<super::ephemeral_host::ApplicationHost>(request)
        .await
}
#[cfg(test)]
#[path = "ephemeral_task_tests.rs"]
mod tests;
