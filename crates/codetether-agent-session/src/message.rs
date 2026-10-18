//! Refresh a live child's access policy before executing a message.
use crate::ChildSession;
use anyhow::{Context, Result};
use codetether_agent_input::Params;
/// Prepare an existing child for a message under the current parent access ceiling.
/// # Arguments
/// * `agent_id` — Child identity used in the missing-session error.
/// * `params` — Current parent policy and identity.
/// * `session` — Owner-scoped native lookup result.
/// # Returns
/// The session with inherited policy refreshed and a bus attached if absent.
/// # Errors
/// Returns the existing missing-child error when lookup yielded none.
/// # Examples
/// Use [ChildSession] for native session and parent-policy operations.
pub async fn prepare_message<S: ChildSession>(
    agent_id: &str,
    params: &Params,
    session: Option<S>,
) -> Result<S> {
    let mut session = session.context(format!("Agent {agent_id} not found"))?;
    *session.settings().inherited_prior_context_allowed = Some(
        crate::parent_prior_context_allowed::<S>(
            params.parent_prior_context_allowed,
            params.parent_session_id.as_deref(),
        )
        .await,
    );
    session.ensure_bus();
    Ok(session)
}
