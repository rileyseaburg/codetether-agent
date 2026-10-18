//! Native output-buffer and event bindings for shared assistant publication.
use super::state::State;

pub(super) fn record(state: &mut State, text: &str) {
    codetether_swarm_core::agent_loop::events::record_output(
        &mut state.inner.conversation.output,
        state.inner.conversation.steps,
        state.inner.events.as_ref(),
        &state.inner.subtask_id,
        text,
    );
}
