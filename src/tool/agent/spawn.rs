//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy.

use super::params::Params;
use super::session_factory;
use super::spawn_request::SpawnRequest;
use super::spawn_store;
use super::spawn_validation;
use crate::tool::ToolResult;
use anyhow::Result;

/// Spawns a new sub-agent after validating the request.
pub(super) async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    let request = SpawnRequest::from_params(params)?;
    if let Err(result) = spawn_validation::validate_spawn_request(&request).await {
        return Ok(result);
    }
    let session = session_factory::create_agent_session(
        request.name,
        request.instructions,
        request.model,
        request.parent_workspace.clone(),
    )
    .await?;
    if request.ephemeral {
        tracing::info!(agent = %request.name, model = %request.model, "Ephemeral sub-agent spawned");
        return Ok(ToolResult::success(ephemeral_message(&request)));
    }
    if let Err(error) =
        spawn_store::persist_spawned_agent(request.name, request.instructions, session).await
    {
        return Ok(ToolResult::error(failure_message(&request, &error)));
    }
    tracing::info!(agent = %request.name, model = %request.model, "Sub-agent spawned");
    Ok(ToolResult::success(success_message(&request)))
}

fn success_message(request: &SpawnRequest<'_>) -> String {
    format!(
        "Spawned @{} on '{}': {}",
        request.name, request.model, request.instructions
    )
}

fn ephemeral_message(request: &SpawnRequest<'_>) -> String {
    format!(
        "{}\nwarning: ephemeral: true; child session was not persisted",
        success_message(request)
    )
}

fn failure_message(request: &SpawnRequest<'_>, error: &anyhow::Error) -> String {
    format!(
        "Failed to spawn @{} durably: child session persistence failed: {error}",
        request.name
    )
}
