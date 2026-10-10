use super::{background, picker, state::State};
use codetether_companion_desktop::{check_available, validate_selection};

pub(super) fn invalidate(state: &mut State) {
    background::cancel(state);
    state.desktop_available = false;
    state.background.suspended = true;
    if !state.background.paused {
        state.status = "Sharing suspended — desktop changed.";
    }
}

pub(super) fn check(state: &mut State, session_blocked: bool) {
    let available = !session_blocked && check_available().is_ok();
    if !available {
        invalidate(state);
        return;
    }
    if state
        .selected
        .as_ref()
        .is_some_and(|m| validate_selection(m).is_err())
    {
        picker::refresh(state);
        state.status = "Sharing OFF — monitor changed; select the shared monitor again.";
    }
    if state.background.suspended && !state.background.paused && state.device.is_some() {
        state.status = "Desktop available — waiting for the shared monitor.";
    }
    state.background.suspended = false;
    state.desktop_available = true;
}
