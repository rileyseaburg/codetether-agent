//! Watchdog-triggered request restart for stalled prompts.
//!
//! Detects when the provider has stopped sending events for longer than the
//! configured timeout and automatically restarts the request.
//!
//! This module handles the **watchdog-timer branch** of the select loop.
//! The per-tick fast-path lives in [`super::tick_watchdog`], and the shared
//! state reset lives in [`watchdog_state`].

#[path = "watchdog_state.rs"]
mod watchdog_state;

use std::time::Duration;

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;

pub(in crate::tui::app::event_loop) use watchdog_state::apply_watchdog_state;

/// Check the watchdog and restart if stalled.
pub(super) async fn maybe_watchdog_restart(
    app: &mut App,
    runtime: &TuiSessionHandle,
    watchdog_interval: Duration,
) {
    let notif = match crate::tui::app::watchdog::check_watchdog_stall(&app.state, watchdog_interval)
    {
        Some(n) => n,
        None => return,
    };
    let prompt = app
        .state
        .main_watchdog_root_prompt
        .clone()
        .or_else(|| app.state.main_inflight_prompt.clone());
    if prompt.is_none() {
        return;
    }
    apply_watchdog_state(app, notif);
    runtime.cancel_current().await;
}
