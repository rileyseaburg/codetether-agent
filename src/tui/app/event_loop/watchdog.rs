//! Timer-driven watchdog checks with native UI and runtime bindings.
#[path = "watchdog_ui.rs"]
mod ui;
use crate::tui::{
    app::{session_runtime::TuiSessionHandle, state::App},
    constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
};
use std::time::Duration;
impl codetether_watchdog::recovery::CheckHost for App {
    fn detect(&self, interval: Duration) -> Option<codetether_watchdog::WatchdogNotification> {
        crate::tui::app::watchdog::check_watchdog_stall(&self.state, interval)
    }
    fn has_inflight(&self) -> bool {
        self.state.main_inflight_prompt.is_some()
    }
}
/// Check the timer watchdog and cancel a stalled prompt eligible for retry.
pub(super) async fn maybe_watchdog_restart(
    app: &mut App,
    runtime: &TuiSessionHandle,
    watchdog_interval: Duration,
) {
    codetether_watchdog::recovery::check(
        app,
        watchdog_interval,
        MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
        true,
        || runtime.cancel_current(),
    )
    .await;
}
