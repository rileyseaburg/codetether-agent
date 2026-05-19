//! Per-message formatted-line cache.
//!
//! Thread-local cache keyed by `(timestamp_nanos, content_len, max_width,
//! role)`. `ChatMessage` content is immutable post-construction, so formatted
//! lines can be reused across frames until width or content changes.

use std::cell::RefCell;
use std::collections::HashMap;

use ratatui::text::Line;

use crate::tui::chat::message::ChatMessage;
use crate::tui::message_formatter::MessageFormatter;

type Key = (u128, usize, usize, u8);
const CACHE_CAP: usize = 128;

thread_local! {
    static FORMAT_CACHE: RefCell<HashMap<Key, Vec<Line<'static>>>> =
        RefCell::new(HashMap::with_capacity(128));
}

fn role_discrim(r: &str) -> u8 {
    match r {
        "user" => 1,
        "assistant" => 2,
        "system" => 3,
        "error" => 4,
        _ => 0,
    }
}

fn ts_nanos(m: &ChatMessage) -> u128 {
    m.timestamp
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
}

/// Format `message.content` for `role` at `max_width`, reusing prior parses.
pub fn format_message_cached(
    message: &ChatMessage,
    role: &str,
    formatter: &MessageFormatter,
    max_width: usize,
) -> Vec<Line<'static>> {
    let key = (
        ts_nanos(message),
        message.content.len(),
        max_width,
        role_discrim(role),
    );
    FORMAT_CACHE.with(|c| {
        if let Some(l) = c.borrow().get(&key) {
            return l.clone();
        }
        let lines = formatter.format_content(&message.content, role);
        let mut m = c.borrow_mut();
        if m.len() >= CACHE_CAP {
            m.clear();
        }
        m.insert(key, lines.clone());
        lines
    })
}

/// Clear the entire cache.
pub fn reset_format_cache() {
    FORMAT_CACHE.with(|c| c.borrow_mut().clear());
}
