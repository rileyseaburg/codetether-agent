//! Assemble live assistant headers and cached streaming body lines.
use codetether_message_formatter::MessageFormatter;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Append the current assistant preview while processing nonempty text.
///
/// # Arguments
/// * `lines` - Destination transcript rows.
/// * `processing` - Whether the turn is active.
/// * `text` - Cumulative assistant text.
/// * `separator_width` - Requested separator width.
/// * `formatter` - Message formatter.
/// * `elapsed` - Lazy current elapsed milliseconds for the header.
///
/// # Returns
/// Appends the header and formatted body, or leaves lines unchanged when idle.
///
/// # Examples
/// ```
/// let mut lines = vec![];
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// codetether_tool_panel::push_streaming_preview(&mut lines, false, "", 60, &formatter, || None);
/// assert!(lines.is_empty());
/// ```
pub fn push_streaming_preview(
    lines: &mut Vec<Line<'static>>,
    processing: bool,
    text: &str,
    separator_width: usize,
    formatter: &MessageFormatter,
    elapsed: impl FnOnce() -> Option<u64>,
) {
    if !processing || text.is_empty() {
        return;
    }
    let (sep, header) = codetether_chat_status::streaming_header(
        || codetether_chat_status::elapsed_badge(elapsed()),
        separator_width,
    );
    lines.push(sep);
    lines.push(header);
    for line in codetether_message_cache::cached_streaming_format(text, formatter) {
        let mut spans = vec![Span::styled("  ", Style::default().fg(Color::Cyan))];
        spans.extend(line.spans);
        lines.push(Line::from(spans));
    }
}
