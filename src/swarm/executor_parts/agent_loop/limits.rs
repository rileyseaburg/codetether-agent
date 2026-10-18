//! Shared step and deadline policy applied to application state.
use super::{super::AgentLoopExit, state::State};
pub(super) fn reached(state: &State) -> Option<AgentLoopExit> {
    codetether_swarm_core::agent_loop::limit_reached(
        state.conversation.steps,
        state.max_steps,
        state.conversation.deadline,
        state.timeout_secs,
    )
}
