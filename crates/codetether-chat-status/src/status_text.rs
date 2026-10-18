//! Status text span (yellow when processing, green when idle).

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Render status text with processing or idle color.
///
/// # Arguments
/// * `status` - Status text to copy.
/// * `processing` - Whether a turn is running.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::status_text::status_text_span("Ready", false);
/// ```
pub fn status_text_span(status: &str, processing: bool) -> Span<'static> {
    Span::styled(
        status.to_string(),
        Style::default().fg(if processing {
            Color::Yellow
        } else {
            Color::Green
        }),
    )
}
