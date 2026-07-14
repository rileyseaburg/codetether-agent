//! Surface a permanent watchdog give-up error to the user.
//!
//! When the retry budget is exhausted, the TUI stops retrying and shows
//! a clear message so the user can take manual action (switch providers,
//! cancel, or re-send the prompt).

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::constants::WATCHDOG_MAX_RESTARTS;

pub(super) fn surface(app: &mut App) {
    let max = WATCHDOG_MAX_RESTARTS;
    app.state.watchdog_notification = None;
    app.state.processing = false;
    app.state.clear_streaming_text();
    app.state.clear_request_timing();
    app.state.status = format!("Watchdog gave up after {max} retries");
    app.state.messages.push(ChatMessage::new(
        MessageType::Error,
        format!(
            "⚠ Watchdog: request stalled {max} times. The provider or tool \
             may be permanently unavailable.\n\nPress Enter to retry \
             manually or switch providers with Ctrl+M."
        ),
    ));
    app.state.scroll_to_bottom();
}
