//! Per-tick watchdog check.
//!
//! Called on every 100 ms UI tick so stalls are detected within one tick
//! rather than waiting for the coarse watchdog interval. When a stall is
//! detected the request is cancelled immediately; the retry submission
//! happens on the next loop iteration via `watchdog_retry`.

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;

pub(in crate::tui::app::event_loop) async fn check(
    app: &mut App,
    runtime: &TuiSessionHandle,
    interval: std::time::Duration,
) {
    codetether_watchdog::recovery::check(
        app,
        interval,
        crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
        false,
        || runtime.cancel_current(),
    )
    .await;
}
