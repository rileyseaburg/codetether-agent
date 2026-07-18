//! Transactional durable-child creation and first-turn dispatch.

use super::super::{params::Params, spawn_messages::failure_message, spawn_request::SpawnRequest};
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn run(
    params: &Params,
    request: &SpawnRequest<'_>,
    prepared: super::prepare::Prepared,
) -> Result<ToolResult> {
    let slot = match super::super::residency::reserve(None).await {
        Ok(slot) => slot,
        Err(result) => return Ok(result),
    };
    let session = super::session::create(params, request).await?;
    let agent_id = match super::super::spawn_store::persist_spawned_agent(request, session).await {
        Ok(agent_id) => agent_id,
        Err(error) => return Ok(ToolResult::error(failure_message(request, &error))),
    };
    slot.commit(&agent_id);
    let warning = prepared.commit();
    super::finish::run(request, &agent_id, warning.as_deref()).await
}
