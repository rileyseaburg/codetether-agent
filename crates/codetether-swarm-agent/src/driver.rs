//! Application entrypoint to the shared sub-agent turn driver.
use super::state::State;
use crate::AgentLoopExit;
use anyhow::Result;
/// Execute provider turns and tool batches against the retained state.
/// # Arguments
/// `state` contains the active conversation, registry and limits.
/// # Returns
/// The original shared driver's termination reason.
/// # Errors
/// Propagates provider, policy and execution errors.
pub async fn execute<H: crate::Host>(state: &mut State<H>) -> Result<AgentLoopExit> {
    codetether_swarm_core::agent_loop::execute(state).await
}
