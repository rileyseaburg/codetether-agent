//! Dispatch inline emphasis and math delimiters.
use crate::inline_state::InlineState;
use ratatui::text::Span;
impl crate::MessageFormatter {
    pub(crate) fn format_inline_text(&self, line: &str, role: &str) -> Vec<Span<'static>> {
        let mut state = InlineState::new(role);
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        crate::inline_bold::consume(&mut state, &mut chars);
                    } else {
                        crate::inline_italic::consume(&mut state);
                    }
                }
                '`' => crate::inline_code::consume(&mut state),
                '\\' if chars.peek() == Some(&'(') => {
                    crate::inline_math::consume(&mut state, &mut chars)
                }
                _ => state.current.push(c),
            }
        }
        crate::inline_finish::finish(state, line)
    }
}
