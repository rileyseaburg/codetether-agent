//! header spans for the status bar.
use super::status_hints::{keybinding_spans, session_label_spans};
use ratatui::text::Span;

/// Session heading combined with the full keyboard hints.
///
/// # Arguments
/// * `session_label` - Display label for the active session.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::status_hints::header_spans("session");
/// ```
pub fn header_spans(session_label: &str) -> Vec<Span<'static>> {
    let mut spans = keybinding_spans();
    spans.push(Span::raw(" | "));
    spans.extend(session_label_spans(session_label));
    spans.push(Span::raw(" | "));
    spans
}
