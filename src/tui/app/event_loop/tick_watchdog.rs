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
    let Some(notif) =
        crate::tui::app::watchdog::check_watchdog_stall(&app.state, interval)
    else {
        return;
    };
    super::super::watchdog::apply_watchdog_state(app, notif);
    runtime.cancel_current().await;
}
