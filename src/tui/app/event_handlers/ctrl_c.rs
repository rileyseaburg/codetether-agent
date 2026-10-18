//! Delegate native interruption keys to shared UI event policy.
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};
pub(super) fn handle_ctrl_c(app: &mut App, runtime: &TuiSessionHandle) -> bool {
    codetether_tui_navigation::events::interrupt::handle_ctrl_c(
        &mut super::interrupt_host::Native { app, runtime },
    )
}
