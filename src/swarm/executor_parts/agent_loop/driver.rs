//! Application entrypoint to the shared sub-agent turn driver.
use super::super::AgentLoopExit;
use super::state::State;
use anyhow::Result;
pub(super) async fn execute(state: &mut State) -> Result<AgentLoopExit> {
    codetether_swarm_core::agent_loop::execute(state).await
}
