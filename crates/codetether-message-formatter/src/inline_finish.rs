//! Flush pending inline text and preserve empty input styling.
use crate::inline_state::InlineState;
use ratatui::{style::Style, text::Span};
pub(crate) fn finish(mut state: InlineState, line: &str) -> Vec<Span<'static>> {
    if !state.current.is_empty() {
        state.spans.push(Span::styled(
            state.current,
            Style::default().fg(state.role_color),
        ));
    }

    if state.spans.is_empty() {
        state.spans.push(Span::styled(
            line.to_string(),
            Style::default().fg(state.role_color),
        ));
    }

    state.spans
}
