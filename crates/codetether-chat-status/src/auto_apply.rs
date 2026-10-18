//! Auto-apply ON/OFF badge spans.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Render the auto-apply state.
///
/// # Arguments
/// * `enabled` - Whether automatic edits are enabled.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::auto_apply::auto_apply_spans(true);
/// ```
pub fn auto_apply_spans(enabled: bool) -> Vec<Span<'static>> {
    vec![
        Span::styled("auto-apply", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(
            if enabled { "ON" } else { "OFF" },
            Style::default().fg(if enabled { Color::Green } else { Color::Yellow }),
        ),
    ]
}
