//! Per-message formatting and full-frame rendered-line caches.
//!
//! [RenderedLineCache] owns the UI frame and frozen streaming prefix.
//! [RenderInputs] supplies only the state needed to validate those cached lines.
//!
//! Thread-local cache keyed by `(timestamp_nanos, content_address, content_len,
//! max_width,
//! role)`. `ChatMessage` content is immutable post-construction, so formatted
//! lines can be reused across frames until width or content changes.

use std::cell::RefCell;

use ratatui::text::Line;

use codetether_chat_model::ChatMessage;
use codetether_message_formatter::MessageFormatter;

mod policy;
mod store;

use store::FormatCache;

type Key = (u128, usize, usize, usize, u8);

thread_local! {
    static FORMAT_CACHE: RefCell<FormatCache> = RefCell::new(FormatCache::new());
}

/// Format `message.content` for `role` at `max_width`, reusing prior parses.
///
/// # Arguments
/// * `message` - Immutable chat message supplying content and cache identity.
/// * `role` - Role name controlling display formatting.
/// * `formatter` - Formatter configured for the supplied width.
/// * `max_width` - Width included in the cache key.
///
/// # Returns
/// Owned formatted lines, cloned from the thread-local cache when available.
///
/// # Examples
/// ```no_run
/// # fn example(message: &codetether_chat_model::ChatMessage) {
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// let _lines = codetether_message_cache::format_message_cached(message, "assistant", &formatter, 80);
/// # }
/// ```
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
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Clears retained entries and byte accounting on the current thread.
///
/// # Examples
/// ```
/// codetether_message_cache::reset_format_cache();
/// ```
pub fn reset_format_cache() {
    FORMAT_CACHE.with(|cache| cache.borrow_mut().clear());
}

mod render_access;
mod render_frozen;
mod render_inputs;
mod render_state;
mod render_store;
mod render_validity;
pub use render_inputs::RenderInputs;
pub use render_state::RenderedLineCache;

mod drawn_lines;
pub use drawn_lines::DrawnLines;

mod streaming;
pub use streaming::{cached_streaming_format, reset_stream_parse_cache};

/// Focus selection and full-frame chat-line assembly.
pub mod pipeline;
