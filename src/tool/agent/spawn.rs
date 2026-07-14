//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy. After persistence succeeds, the first
//! model turn is auto-started via [`super::spawn_run`] so the sub-agent begins
//! working immediately instead of sitting idle (issue #295).

use super::params::Params;
use super::session_factory;
use super::spawn_messages::failure_message;
use super::spawn_request::SpawnRequest;
use super::spawn_store;
use super::spawn_validation;
use crate::tool::ToolResult;
use anyhow::Result;

#[path = "spawn/ephemeral.rs"]
mod ephemeral;
#[path = "spawn/ephemeral_result.rs"]
mod ephemeral_result;
#[path = "spawn/ephemeral_setup.rs"]
mod ephemeral_setup;
#[path = "spawn/finish.rs"]
mod finish;

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
    if request.ephemeral {
        return ephemeral::run(&request, warning.as_deref()).await;
    }
    let prior_context_allowed = session_factory::parent_prior_context_allowed(
        params.parent_prior_context_allowed,
        request.parent_session_id,
    )
    .await;
    let session = session_factory::create_agent_session(
        request.name,
        request.instructions,
        request.model,
        request.parent_workspace.clone(),
        prior_context_allowed,
    )
    .await?;
    if let Err(error) = spawn_store::persist_spawned_agent(&request, session).await {
        return Ok(ToolResult::error(failure_message(&request, &error)));
    }
    finish::run(&request, warning.as_deref()).await
}
