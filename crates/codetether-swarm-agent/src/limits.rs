//! Shared step and deadline policy applied to application state.
use super::state::State;
use crate::AgentLoopExit;
pub(super) fn reached<H: crate::Host>(state: &State<H>) -> Option<AgentLoopExit> {
    codetether_swarm_core::agent_loop::limit_reached(
        state.inner.conversation.steps,
        state.inner.max_steps,
        state.inner.conversation.deadline,
        state.inner.timeout_secs,
    )
}
