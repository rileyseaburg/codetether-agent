//! Toggle inline code state after a plain-text flush.
use crate::inline_state::InlineState;
use ratatui::{style::Style, text::Span};
pub(crate) fn consume(state: &mut InlineState) {
    if !state.current.is_empty() {
        state.spans.push(Span::styled(
            state.current.clone(),
            Style::default().fg(state.role_color),
        ));
        state.current.clear();
    }
    state.in_code = !state.in_code;
}
