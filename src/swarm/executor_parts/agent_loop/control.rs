//! Shared runtime boundaries for delegated provider and tool work.

use super::state::State;

pub(super) async fn ready(state: &State) -> anyhow::Result<()> {
    if let Some(control) = &state.control {
        control.ready().await?;
    }
    Ok(())
}

pub(super) fn apply_input(state: &mut State) -> bool {
    let Some(control) = &state.control else {
        return false;
    };
    let pending = control.inputs_since(&mut state.steering_cursor);
    let received = !pending.is_empty();
    state.messages.extend(pending);
    received
}
