//! Per-message formatted-line cache.
//!
//! Thread-local cache keyed by `(timestamp_nanos, content_address, content_len,
//! max_width,
//! role)`. `ChatMessage` content is immutable post-construction, so formatted
//! lines can be reused across frames until width or content changes.

use std::cell::RefCell;

use ratatui::text::Line;

use crate::tui::chat::message::ChatMessage;
use crate::tui::message_formatter::MessageFormatter;

mod policy;
mod store;

use store::FormatCache;

type Key = (u128, usize, usize, usize, u8);

thread_local! {
    static FORMAT_CACHE: RefCell<FormatCache> = RefCell::new(FormatCache::new());
}

/// Format `message.content` for `role` at `max_width`, reusing prior parses.
pub fn format_message_cached(
    message: &ChatMessage,
    role: &str,
    formatter: &MessageFormatter,
    max_width: usize,
) -> Vec<Line<'static>> {
    let key = policy::key(message, role, max_width);
    FORMAT_CACHE.with(|c| {
        if let Some(lines) = c.borrow().get(&key) {
            return lines.clone();
        }
        let lines = formatter.format_content(&message.content, role);
        c.borrow_mut().insert(key, lines.clone());
        lines
    })
}

/// Clear the entire cache.
pub fn reset_format_cache() {
    FORMAT_CACHE.with(|cache| cache.borrow_mut().clear());
}
