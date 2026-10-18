//! Toggle bold after flushing buffered text.
use crate::inline_state::InlineState;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};
use std::{iter::Peekable, str::Chars};
pub(crate) fn consume(state: &mut InlineState, chars: &mut Peekable<Chars<'_>>) {
    // Bold
    if !state.current.is_empty() {
        state.spans.push(Span::styled(
            state.current.clone(),
            Style::default()
                .fg(state.role_color)
                .add_modifier(if state.in_bold {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ));
        state.current.clear();
    }
    chars.next(); // consume second '*'
    state.in_bold = !state.in_bold;
}
