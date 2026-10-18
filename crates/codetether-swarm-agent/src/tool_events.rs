//! Native event-channel bindings for shared tool activity publication.
use super::state::State;

pub(super) fn started<H: crate::Host>(state: &State<H>, name: &str) {
    codetether_swarm_core::agent_loop::events::started(
        state.inner.events.as_ref(),
        &state.inner.subtask_id,
        name,
    );
}

pub(super) fn finished<H: crate::Host>(
    state: &State<H>,
    name: &str,
    input: &str,
    output: &str,
    success: bool,
) {
    codetether_swarm_core::agent_loop::events::finished(
        state.inner.events.as_ref(),
        &state.inner.subtask_id,
        name,
        input,
        output,
        success,
    );
}
