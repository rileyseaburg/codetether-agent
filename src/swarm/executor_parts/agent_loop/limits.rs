//! Step and deadline enforcement for a sub-agent loop.

use super::{super::AgentLoopExit, state::State};
use std::time::Instant;

pub(super) fn reached(state: &State) -> Option<AgentLoopExit> {
    if state.steps >= state.max_steps {
        tracing::warn!(max_steps = state.max_steps, "Sub-agent reached step limit");
        Some(AgentLoopExit::MaxStepsReached)
    } else if Instant::now() > state.deadline {
        tracing::warn!(timeout_secs = state.timeout_secs, "Sub-agent timed out");
        Some(AgentLoopExit::TimedOut)
    } else {
        None
    }
}
