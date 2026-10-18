//! Shared test fixtures for watchdog detector tests.

use std::time::Duration;

use crate::tui::app::state::AppState;

pub(super) const TIMEOUT: Duration = Duration::from_secs(60);

/// An [`AppState`] in the processing state, ready for stall scenarios.
pub(super) fn processing_state() -> AppState {
    let mut state = AppState::default();
    state.processing = true;
    state
}
