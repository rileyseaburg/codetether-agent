//! Shared runtime boundaries for delegated provider and tool work.

use super::state::State;

pub(super) async fn ready(state: &State) -> anyhow::Result<()> {
    codetether_session_control::delegated::ready(state.control.as_ref()).await
}

pub(super) fn apply_input(state: &mut State) -> bool {
    codetether_session_control::delegated::apply_input(
        state.control.as_ref(),
        &mut state.steering_cursor,
        &mut state.messages,
    )
}
