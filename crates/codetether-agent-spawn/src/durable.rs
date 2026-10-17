//! Transactional durable-child creation and first-turn dispatch.

use crate::{Host, messages::failure_message};
use anyhow::Result;
use codetether_agent_input::{Params, SpawnRequest};
use codetether_tool_core::ToolResult;

pub(super) async fn run<H: Host>(
    params: &Params,
    request: &SpawnRequest<'_>,
    prepared: super::prepare::Prepared,
) -> Result<ToolResult> {
    let slot = match H::reserve_capacity().await {
        Ok(slot) => slot,
        Err(result) => return Ok(result),
    };
    let (session, handoff) = H::create_session(params, request).await?;
    let agent_id = match H::persist(request, session).await {
        Ok(agent_id) => agent_id,
        Err(error) => {
            return Ok(handoff.attach(ToolResult::error(failure_message(request, &error))));
        }
    };
    slot.commit(&agent_id);
    let warning = prepared.commit();
    let result = super::finish::run::<H>(request, &agent_id, warning.as_deref())
        .await
        .unwrap_or_else(|error| ToolResult::error(format!("Child dispatch failed: {error}")));
    Ok(handoff.attach(result))
}
