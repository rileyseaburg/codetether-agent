//! Wrap styled spans into bounded terminal rows.
use crate::wrap_state::WrapState;
use ratatui::text::{Line, Span};
impl crate::MessageFormatter {
    pub(crate) fn wrap_line(&self, spans: Vec<Span<'static>>, width: usize) -> Vec<Line<'static>> {
        if spans.is_empty() {
            return vec![Line::from("")];
        }
        if width == 0 {
            return vec![Line::from(spans)];
        }
        let mut state = WrapState::default();
        for span in spans {
            crate::wrap_span::append(&mut state, span.style, span.content.into_owned(), width);
        }
        if !state.cur.is_empty() {
            state.out.push(Line::from(state.cur));
        }
        if state.out.is_empty() {
            state.out.push(Line::from(""));
        }
        state.out
    }
}
