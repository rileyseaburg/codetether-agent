//! Construction and parent-context inheritance for a spawned session.

use crate::{ChildSession, workspace_binding};
use anyhow::{Context, Result};
use codetether_agent_input::{Params, SpawnRequest};
use codetether_child_workspace::Handoff;
use codetether_collaboration_history::HistorySource;

/// Create a child in an isolated checkout and append its selected parent history.
/// # Arguments
/// * `params` — Runtime policy input from the parent.
/// * `request` — Validated spawn settings and parent identity.
/// # Returns
/// The configured child and retained checkout handoff.
/// # Errors
/// Propagates session, checkout and history-loading errors, retaining checkout
/// location context when history inheritance fails after allocation.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_session::{create_isolated, ChildSession};
/// use codetether_collaboration_history::HistorySource;
/// use codetether_agent_input::{Params, SpawnRequest};
/// async fn example<S: ChildSession + HistorySource>(params: &Params) -> anyhow::Result<()> {
///     let request = SpawnRequest::from_params(params)?;
///     let _ = create_isolated::<S>(params, &request).await?;
///     Ok(())
/// }
/// ```
pub async fn create_isolated<S: ChildSession + HistorySource>(
    params: &Params,
    request: &SpawnRequest<'_>,
) -> Result<(S, Handoff)> {
    let allowed = crate::parent_prior_context_allowed::<S>(
        params.parent_prior_context_allowed,
        request.parent_session_id,
    )
    .await;
    let mut session = crate::create::<S>(
        request.name,
        request.instructions,
        request.model,
        request.parent_workspace.clone(),
        allowed,
    )
    .await?;
    let requested = session
        .settings()
        .directory
        .clone()
        .context("Child session has no working directory")?;
    let handoff = codetether_child_workspace::allocate(&requested).await?;
    workspace_binding::bind(&mut session, request.name, request.instructions, &handoff);
    codetether_collaboration_history::inherit(
        &mut session,
        request.parent_session_id,
        request.fork_turns,
    )
    .await
    .with_context(|| format!("Child checkout retained at {}", handoff.worktree.display()))?;
    workspace_binding::remind(&mut session, &handoff);
    Ok((session, handoff))
}
