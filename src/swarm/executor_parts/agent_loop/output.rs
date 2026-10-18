//! Native output-buffer and event bindings for shared assistant publication.
use super::state::State;

pub(super) fn record(state: &mut State, text: &str) {
    codetether_swarm_core::agent_loop::events::record_output(
        &mut state.output,
        state.steps,
        state.events.as_ref(),
        &state.subtask_id,
        text,
    );
}
