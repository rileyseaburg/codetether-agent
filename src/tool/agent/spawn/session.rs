//! Construction and parent-context inheritance for a spawned session.

use super::super::{params::Params, session_factory, spawn_request::SpawnRequest};
use crate::session::Session;
use anyhow::Result;

pub(super) async fn create(params: &Params, request: &SpawnRequest<'_>) -> Result<Session> {
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
    super::super::collaboration_runtime::fork_context::inherit(
        &mut session,
        request.parent_session_id,
        request.fork_turns,
    )
    .await?;
    Ok(session)
}
