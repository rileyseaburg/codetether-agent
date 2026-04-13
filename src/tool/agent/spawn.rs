//! Spawn orchestration for the agent tool.
//!
//! This module coordinates spawn request parsing, validation, session
//! creation, and persistence. It intentionally permits sub-agents to
//! use the same model as their parent.
//!
//! # Examples
//!
//! ```ignore
//! let result = handle_spawn(&params).await?;
//! assert!(result.success);
//! ```

use super::params::Params;
use super::session_factory;
use super::spawn_request::SpawnRequest;
use super::spawn_store;
use super::spawn_validation;
use crate::tool::ToolResult;
use anyhow::Result;

/// Spawns a new sub-agent after validating the request.
///
/// # Examples
///
/// ```ignore
/// let result = handle_spawn(&params).await?;
/// ```
pub(super) async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    let request = SpawnRequest::from_params(params)?;
    if let Err(result) = spawn_validation::validate_spawn_request(&request).await {
        return Ok(result);
    }
    let session =
        session_factory::create_agent_session(request.name, request.instructions, request.model)
            .await?;
    spawn_store::persist_spawned_agent(request.name, request.instructions, session).await;
    tracing::info!(agent = %request.name, model = %request.model, "Sub-agent spawned");
    Ok(ToolResult::success(success_message(&request)))
}

fn success_message(request: &SpawnRequest<'_>) -> String {
    format!(
        "Spawned @{} on '{}': {}",
        request.name, request.model, request.instructions
    )
}
