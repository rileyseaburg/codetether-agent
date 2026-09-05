//! Construction and parent-context inheritance for a spawned session.

use super::super::{params::Params, session_factory, spawn_request::SpawnRequest};
use crate::session::Session;
use anyhow::{Context, Result};

pub(super) async fn create(
    params: &Params,
    request: &SpawnRequest<'_>,
) -> Result<(Session, super::workspace::Handoff)> {
    let allowed = session_factory::parent_prior_context_allowed(
        params.parent_prior_context_allowed,
        request.parent_session_id,
    )
    .await;
    let mut session = session_factory::create_agent_session(
        request.name,
        request.instructions,
        request.model,
        request.parent_workspace.clone(),
        allowed,
    )
    .await?;
    let requested = session
        .metadata
        .directory
        .clone()
        .context("Child session has no working directory")?;
    let handoff = super::workspace::allocate(&requested).await?;
    super::session_workspace::bind(&mut session, request.name, request.instructions, &handoff);
    super::super::collaboration_runtime::fork_context::inherit(
        &mut session,
        request.parent_session_id,
        request.fork_turns,
    )
    .await
    .with_context(|| format!("Child checkout retained at {}", handoff.worktree.display()))?;
    super::session_workspace::remind(&mut session, &handoff);
    Ok((session, handoff))
}

#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
