//! Toggle italic after flushing buffered text.
use crate::inline_state::InlineState;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};
pub(crate) fn consume(state: &mut InlineState) {
    // Italic
    if !state.current.is_empty() {
        state.spans.push(Span::styled(
            state.current.clone(),
            Style::default()
                .fg(state.role_color)
                .add_modifier(if state.in_italic {
                    Modifier::ITALIC
                } else {
                    Modifier::empty()
                }),
        ));
        state.current.clear();
    }
    state.in_italic = !state.in_italic;
}
