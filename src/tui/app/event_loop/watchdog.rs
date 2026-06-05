//! Watchdog-triggered request restart for stalled prompts.
//!
//! Detects when the provider has stopped sending events for
//! longer than the configured timeout and automatically
//! restarts the request with the same prompt.
//!
//! # Examples
//!
//! ```ignore
//! maybe_watchdog_restart(
//!     &mut app, session, &registry, &tx, &rtx, interval,
//! ).await;
//! ```

use std::time::Duration;

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;

/// Check the watchdog and restart if stalled.
///
/// # Examples
///
/// ```ignore
/// maybe_watchdog_restart(&mut app, session, &reg, &tx, &rtx, interval).await;
/// ```
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

/// Reset app state for the watchdog restart.
fn apply_watchdog_state(app: &mut App, notif: crate::tui::app::watchdog::WatchdogNotification) {
    app.state.main_watchdog_restart_count += 1;
    let count = app.state.main_watchdog_restart_count;
    app.state.watchdog_notification = Some(notif);
    app.state.processing = false;
    app.state.streaming_text.clear();
    app.state.clear_request_timing();
    app.state.status = format!("Watchdog timeout — restarting request (attempt {count})");
    app.state.messages.push(ChatMessage::new(
        MessageType::Error,
        format!(
            "⚠ Watchdog: no events for {MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS}s. \
             Auto-restarting (attempt {count})."
        ),
    ));
    app.state.scroll_to_bottom();
}
