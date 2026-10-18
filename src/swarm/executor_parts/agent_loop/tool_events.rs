//! Native event-channel bindings for shared tool activity publication.
use super::state::State;

pub(super) fn started(state: &State, name: &str) {
    codetether_swarm_core::agent_loop::events::started(
        state.events.as_ref(),
        &state.subtask_id,
        name,
    );
}

pub(super) fn finished(state: &State, name: &str, input: &str, output: &str, success: bool) {
    codetether_swarm_core::agent_loop::events::finished(
        state.events.as_ref(),
        &state.subtask_id,
        name,
        input,
        output,
        success,
    );
}
