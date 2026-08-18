//! Retain only a bounded chat window in live TUI memory.

use crate::tui::app::state::App;
use crate::tui::retained_payload::{CHAT_EXPANDED_MAX_ITEMS, CHAT_RETAINED_MAX_ITEMS};

pub(super) fn trim(app: &mut App) {
    let max_items = retained_limit(app.state.history_page.expanded());
    let len = app.state.messages.len();
    if len <= max_items {
        return;
    }
    let overflow = len - max_items;
    app.state.messages.drain(0..overflow);
    app.state.cached_message_lines.clear();
    app.state.cached_messages_len = 0;
    app.state.cached_frozen_len = 0;
    // Preserve the auto-follow sentinel (SCROLL_BOTTOM = 1_000_000). If the user
    // is at the sentinel, keep them there — the render clamp adjusts to the new
    // max_scroll. For a manual scroll position, subtract the number of removed
    // leading messages so the view stays on the same logical content.
    if app.state.chat_scroll < crate::tui::constants::SCROLL_BOTTOM {
        app.state.chat_scroll = app.state.chat_scroll.saturating_sub(overflow);
    }
}

fn retained_limit(expanded: bool) -> usize {
    if expanded {
        CHAT_EXPANDED_MAX_ITEMS
    } else {
        CHAT_RETAINED_MAX_ITEMS
    }
}

#[cfg(test)]
#[path = "retention_tests.rs"]
mod tests;
