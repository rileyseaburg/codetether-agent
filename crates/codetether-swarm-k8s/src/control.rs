//! Observe shared cancellation while remote pods are being polled.

pub(super) async fn cancelled(control: Option<&codetether_session_control::ControlHandle>) {
    match control {
        Some(control) => control.cancelled().await,
        None => std::future::pending().await,
    }
}

pub(super) fn stopped(control: Option<&codetether_session_control::ControlHandle>) -> bool {
    control.is_some_and(|control| control.is_cancelled() || control.has_input())
}

pub(super) fn may_spawn(state: &super::state::State<'_, impl crate::Host>) -> bool {
    state.active.len() < state.budget
        && !state.executor.control().is_some_and(|control| {
            control.is_cancelled() || control.is_paused() || control.has_input()
        })
}
