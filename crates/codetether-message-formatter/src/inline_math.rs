//! Consume bracketed inline math or retain an unclosed delimiter.
use crate::inline_state::InlineState;
use codetether_math_text::prettify_math;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};
use std::{iter::Peekable, str::Chars};
pub(crate) fn consume(state: &mut InlineState, chars: &mut Peekable<Chars<'_>>) {
    chars.next(); // consume '('
    if !state.current.is_empty() {
        state.spans.push(Span::styled(
            state.current.clone(),
            Style::default().fg(state.role_color),
        ));
        state.current.clear();
    }
    let mut math = String::new();
    let mut closed = false;
    while let Some(mc) = chars.next() {
        if mc == '\\' && chars.peek() == Some(&')') {
            chars.next(); // consume ')'
            closed = true;
            break;
        }
        math.push(mc);
    }
    if closed {
        state.spans.push(Span::styled(
            prettify_math(&math),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::ITALIC),
        ));
    } else {
        // Unclosed: render literally so the source is visible.
        state.current.push_str("\\(");
        state.current.push_str(&math);
    }
}
