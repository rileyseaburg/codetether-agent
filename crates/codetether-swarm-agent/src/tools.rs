//! Sequential execution of tool calls from one provider turn.

use super::state::{State, ToolCall};
mod host;

/// Execute one ordered tool-call batch and append its results.
/// # Arguments
/// `state` owns the conversation; `calls` is the provider's ordered batch.
/// # Returns
/// Whether the shared tool driver requests termination.
/// # Errors
/// Propagates control and execution errors.
pub async fn execute<H: crate::Host>(
    state: &mut State<H>,
    calls: Vec<ToolCall>,
) -> anyhow::Result<bool> {
    codetether_swarm_core::agent_loop::tools::execute(state, calls).await
}
