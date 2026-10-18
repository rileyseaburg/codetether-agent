//! Delegate native interruption keys to shared UI event policy.
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};
pub(super) fn handle(
    app: &mut App,
    runtime: &TuiSessionHandle,
    key: crossterm::event::KeyEvent,
) -> Option<bool> {
    codetether_tui_navigation::events::interrupt::handle(
        &mut super::interrupt_host::Native { app, runtime },
        key,
    )
}
