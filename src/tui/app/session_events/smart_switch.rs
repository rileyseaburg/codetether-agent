//! Native state bindings for shared error-triggered failover scheduling.
use crate::tui::app::{session_runtime::SessionSlot, state::App};
pub(super) fn schedule(app: &mut App, slot: &SessionSlot, err: &str) {
    let state = &mut app.state;
    codetether_model_failover::schedule::schedule(
        &mut state.retry_state,
        slot.view().model.as_deref(),
        &state.model_picker.available_models,
        state.main_inflight_prompt.as_deref(),
        &mut state.status,
        err,
    );
}
