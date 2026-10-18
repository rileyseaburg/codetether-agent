//! Retain only a bounded chat window in live TUI memory.

use crate::{CHAT_EXPANDED_MAX_ITEMS, CHAT_RETAINED_MAX_ITEMS};

/// Retain the newest chat messages and preserve the viewport position.
/// # Arguments
/// * `messages` — Live chronological chat messages.
/// * `expanded` — Whether expanded history uses the larger limit.
/// * `scroll` — Current chat scroll position.
/// * `bottom` — Sentinel representing automatic scrolling to the bottom.
/// * `clear_cache` — Invalidates rendered messages after leading items are removed.
/// # Returns
/// Trims excess messages, invalidates their cache, then adjusts manual scrolling.
/// # Examples
/// ```
/// let mut messages = vec![0; codetether_view_retention::CHAT_RETAINED_MAX_ITEMS + 2];
/// let mut scroll = 5;
/// codetether_view_retention::chat::trim(&mut messages, false, &mut scroll, 1_000_000, || {});
/// assert_eq!(scroll, 3);
/// assert_eq!(messages.len(), codetether_view_retention::CHAT_RETAINED_MAX_ITEMS);
/// ```
pub fn trim<T>(
    messages: &mut Vec<T>,
    expanded: bool,
    scroll: &mut usize,
    bottom: usize,
    clear_cache: impl FnOnce(),
) {
    let max_items = retained_limit(expanded);
    let len = messages.len();
    if len <= max_items {
        return;
    }
    let overflow = len - max_items;
    messages.drain(0..overflow);
    clear_cache();
    // Preserve the auto-follow sentinel (SCROLL_BOTTOM = 1_000_000). If the user
    // is at the sentinel, keep them there — the render clamp adjusts to the new
    // max_scroll. For a manual scroll position, subtract the number of removed
    // leading messages so the view stays on the same logical content.
    if *scroll < bottom {
        *scroll = scroll.saturating_sub(overflow);
    }
}

/// Choose the bounded live-message limit for the current history mode.
/// # Arguments
/// * `expanded` — Whether expanded history is active.
/// # Returns
/// The normal or expanded message limit.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::chat::retained_limit(false), 500);
/// ```
pub fn retained_limit(expanded: bool) -> usize {
    if expanded {
        CHAT_EXPANDED_MAX_ITEMS
    } else {
        CHAT_RETAINED_MAX_ITEMS
    }
}

#[cfg(test)]
#[path = "chat/tests.rs"]
mod tests;
