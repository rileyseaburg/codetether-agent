//! Retain only a bounded chat window in live TUI memory.

use crate::tui::app::state::App;
use crate::tui::retained_payload::CHAT_RETAINED_MAX_ITEMS;

pub(super) fn trim(app: &mut App) {
    let len = app.state.messages.len();
    if len <= CHAT_RETAINED_MAX_ITEMS {
        return;
    }
    let overflow = len - CHAT_RETAINED_MAX_ITEMS;
    app.state.messages.drain(0..overflow);
    app.state.cached_message_lines.clear();
    app.state.cached_messages_len = 0;
    app.state.cached_frozen_len = 0;
    app.state.chat_scroll = app.state.chat_scroll.saturating_sub(overflow);
}
