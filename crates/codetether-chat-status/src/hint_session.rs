//! session label spans for the status bar.
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Session identity label spans.
///
/// # Arguments
/// * `session_label` - Display label for the active session.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::status_hints::session_label_spans("session");
/// ```
pub fn session_label_spans(session_label: &str) -> Vec<Span<'static>> {
    vec![
        Span::styled("session", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(session_label.to_string(), Style::default().fg(Color::Cyan)),
    ]
}
