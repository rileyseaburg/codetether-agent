//! Thread-local parse cache for cumulative streaming text.
use codetether_message_formatter::MessageFormatter;
use ratatui::text::Line;
use std::cell::RefCell;

const STREAM_REPARSE_THRESHOLD: usize = 256;
thread_local! {
    static STREAM_PARSE_CACHE: RefCell<Option<(usize, Vec<Line<'static>>)>> =
        const { RefCell::new(None) };
}

/// Format cumulative assistant text, reusing parses below the growth threshold.
///
/// # Arguments
/// * `text` - Current cumulative stream text.
/// * `formatter` - Formatter used when reparsing is necessary.
///
/// # Returns
/// Owned formatted lines from the most recent parse.
///
/// # Examples
/// ```
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// let _lines = codetether_message_cache::cached_streaming_format("hello", &formatter);
/// ```
pub fn cached_streaming_format(text: &str, formatter: &MessageFormatter) -> Vec<Line<'static>> {
    STREAM_PARSE_CACHE.with(|cell| {
        let cur_len = text.len();
        if let Some((parsed_len, ref lines)) = *cell.borrow()
            && cur_len >= parsed_len
            && cur_len - parsed_len < STREAM_REPARSE_THRESHOLD
        {
            return lines.clone();
        }
        let formatted = formatter.format_content(text, "assistant");
        *cell.borrow_mut() = Some((cur_len, formatted.clone()));
        formatted
    })
}

/// Reset the streaming parse cache.
///
/// Call when a new assistant turn begins so a shrunken buffer doesn't reuse
/// stale parsed lines.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// Clears the current thread's cached streaming parse.
///
/// # Examples
/// ```
/// codetether_message_cache::reset_stream_parse_cache();
/// ```
pub fn reset_stream_parse_cache() {
    STREAM_PARSE_CACHE.with(|cell| *cell.borrow_mut() = None);
}
