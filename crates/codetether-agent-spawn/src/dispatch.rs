//! Parse and route a spawn while retaining its pending identity guard.
use crate::{Host, durable, prepare};
use anyhow::Result;
use codetether_agent_input::{Params, SpawnRequest};
use codetether_tool_core::ToolResult;
/// Validate and spawn a managed child, dispatching its first turn.
/// # Arguments
/// * `params` — Parsed input with current runtime context.
/// # Returns
/// The existing durable or ephemeral result, including admission rejections.
/// # Errors
/// Propagates malformed input and native session/execution setup errors.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_spawn::{Host, handle_spawn};
/// use codetether_agent_input::Params;
/// async fn example<H: Host>(params: &Params) -> anyhow::Result<()> {
///     let _ = handle_spawn::<H>(params).await?;
///     Ok(())
/// }
/// ```

/// Spawns a new sub-agent and auto-starts its first turn.
pub async fn handle_spawn<H: Host>(params: &Params) -> Result<ToolResult> {
    let request = SpawnRequest::from_params(params)?;
    let prepared = match prepare::run::<H>(&request).await {
        Ok(prepared) => prepared,
        Err(result) => return Ok(result),
    };
    if request.ephemeral {
        return H::ephemeral(&request, prepared.warning()).await;
    }
    durable::run::<H>(params, &request, prepared).await
}
