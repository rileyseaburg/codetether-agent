//! Handle the "message" action.

use super::{execution_state, helpers};
use crate::tool::ToolResult;
use anyhow::{Context, Result};

#[path = "remote/mod.rs"]
pub(super) mod remote;
#[path = "message/remote_fallback.rs"]
mod remote_fallback;
#[path = "message/run.rs"]
pub(super) mod run;
#[path = "message/start.rs"]
mod start;
#[path = "message/submit.rs"]
mod submit;
#[path = "message/session.rs"]
mod task_session;

/// Sends a message to an existing spawned sub-agent and returns its response.
///
/// The handler streams intermediate events, persists the updated session, and
/// formats the final reply as structured JSON.
///
/// # Examples
///
/// ```ignore
/// let result = handle_message(&params).await?;
/// ```
pub(super) async fn handle_message(params: &helpers::Params) -> Result<ToolResult> {
    let target = params
        .name
        .as_ref()
        .context("name required for message")?
        .clone();
    let message = params.message.as_ref().context("message required")?.clone();
    let Some((agent_id, entry)) =
        super::store::scope::resolve_for_parent(&target, params.parent_session_id.as_deref())
    else {
        return remote_fallback::execute(&target, &message, params).await;
    };
    if params.detach_or_default() {
        return submit::detached(agent_id, message, params).await;
    }
    let Some(guard) = execution_state::try_start(&agent_id) else {
        let submitted =
            super::collaboration_runtime::message_queue::enqueue(&agent_id, message, params)
                .await?;
        return Ok(ToolResult::success(format!(
            "Queued follow-up for @{} ({agent_id}) at mailbox position {} (submission {})",
            entry.name, submitted.depth, submitted.id
        )));
    };
    run::execute(
        agent_id,
        message,
        params.message_images.clone(),
        params,
        guard,
        None,
    )
    .await
}
