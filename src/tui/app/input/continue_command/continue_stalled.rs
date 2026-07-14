//! Resume a stalled turn for `/continue`.
//!
//! Mirrors the watchdog cancel path but is user-initiated. We record the prompt
//! as the watchdog root prompt, cancel the dead request, and arm the
//! watchdog-retry machinery (`watchdog_retry::execute`) by leaving a
//! notification set. When the runtime returns the cancelled session via a
//! notice, the retry path resubmits the prompt automatically.

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;
use crate::tui::app::watchdog::WatchdogNotification;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Cancel the stalled request and arm an automatic resubmission.
pub(super) async fn resume(app: &mut App, prompt: String, runtime: &TuiSessionHandle) {
    // A manual /continue restores the auto-restart budget so the resubmission
    // is not immediately suppressed by an exhausted watchdog counter.
    app.state.main_watchdog_restart_count = 0;
    app.state.main_watchdog_root_prompt = Some(prompt.clone());
    app.state.main_inflight_prompt = Some(prompt);
    app.state.watchdog_notification = Some(WatchdogNotification::new(
        "Manual /continue — resubmitting stalled turn.".to_string(),
        0,
    ));
    app.state.processing = false;
    app.state.clear_streaming_text();
    app.state.clear_request_timing();
    app.state.status = "Continuing — resubmitting stalled turn…".to_string();
    app.state.messages.push(ChatMessage::new(
        MessageType::System,
        "↻ /continue: cancelling stalled request and resubmitting.".to_string(),
    ));
    app.state.scroll_to_bottom();
    runtime.cancel_current().await;
}
