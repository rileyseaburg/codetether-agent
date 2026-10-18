//! Shared step and deadline policy applied to application state.
use super::{super::AgentLoopExit, state::State};
pub(super) fn reached(state: &State) -> Option<AgentLoopExit> {
    codetether_swarm_core::agent_loop::limit_reached(
        state.inner.conversation.steps,
        state.inner.max_steps,
        state.inner.conversation.deadline,
        state.inner.timeout_secs,
    )
}
