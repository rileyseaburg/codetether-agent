//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy. After persistence succeeds, the first
//! model turn is auto-started via [`super::spawn_run`] so the sub-agent begins
//! working immediately instead of sitting idle (issue #295).

use super::params::Params;
use super::session_factory;
use super::spawn_messages::{ephemeral_message, failure_message, success_message, with_warning};
use super::spawn_request::SpawnRequest;
use super::spawn_run;
use super::spawn_store;
use super::spawn_validation;
use crate::tool::ToolResult;
use anyhow::Result;

/// Spawns a new sub-agent and auto-starts its first turn.
pub(super) async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    let request = SpawnRequest::from_params(params)?;
    let warning = match spawn_validation::validate_spawn_request(&request).await {
        Ok(warning) => warning,
        Err(result) => return Ok(result),
    };
    if let Some(text) = &warning {
        tracing::warn!(agent = %request.name, model = %request.model, "{text}");
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
        return Ok(ToolResult::success(with_warning(
            ephemeral_message(&request),
            warning.as_deref(),
        )));
    }
    if let Err(error) =
        spawn_store::persist_spawned_agent(request.name, request.instructions, session, request.model).await
    {
        return Ok(ToolResult::error(failure_message(&request, &error)));
    }
    tracing::info!(agent = %request.name, model = %request.model, "Sub-agent spawned, auto-starting first turn");
    // Auto-start the first turn so the agent begins working immediately (#295).
    let mut result = spawn_run::kick_off(request.name, request.detach).await?;
    result.output = format!(
        "{}\n\nFirst turn dispatched{}.",
        result.output,
        if request.detach { " (background)" } else { " (synchronous)" }
    );
    let _ = warning;
    Ok(result)
}