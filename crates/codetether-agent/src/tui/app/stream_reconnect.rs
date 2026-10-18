//! Same-model stream-reconnect scheduling and state management.
//!
//! When a prompt fails with a transient stream disconnect (broken pipe,
//! premature EOF, h2 reset, network drop), this module schedules a
//! same-model retry up to [`MAX_RECONNECT_ATTEMPTS`] times before giving up.
//!
//! This is distinct from:
//! - The **watchdog** (handles *stalls* — no events for N seconds).
//! - **Smart-switch** (handles *permanent* provider errors that benefit from
//!   a different model).

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Schedule a stream-reconnect retry if the error is a transient disconnect
/// and the budget has not been exhausted.
///
/// Sets `app.state.pending_stream_reconnect` to the prompt text so the
/// execute path can re-submit it on the next notice-branch iteration.
pub(crate) fn schedule(app: &mut App, error: &str) {
    app.state
        .stream_recovery
        .schedule(error, &app.state.watchdog.inflight_prompt);
}

/// Reset the reconnect counter when a turn completes successfully.
pub(crate) fn reset_on_success(app: &mut App) {
    app.state.stream_recovery.reset_on_success();
}

/// Push a visible reconnect banner to the chat.
pub(crate) fn push_reconnect_banner(app: &mut App) {
    let attempt = app.state.stream_recovery.stream_reconnect_count;
    let msg = codetether_stream_recovery::reconnect_banner(attempt);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, msg));
    app.state.status = codetether_stream_recovery::reconnect_status(attempt);
    app.state.chat_viewport.scroll_to_bottom();
}
