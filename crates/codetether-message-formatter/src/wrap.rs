//! Wrap styled spans into bounded terminal rows.
use crate::wrap_state::WrapState;
use ratatui::text::{Line, Span};
impl crate::MessageFormatter {
    /// Wrap styled spans into rows using terminal display widths.
    ///
    /// # Arguments
    /// * `spans` - Styled text to wrap without losing span styles.
    /// * `width` - Maximum display columns; zero preserves the input row.
    ///
    /// # Returns
    /// Wrapped rows, or one empty row when no spans are supplied.
    ///
    /// # Examples
    /// ```
    /// let formatter = codetether_message_formatter::MessageFormatter::new(80);
    /// assert_eq!(formatter.wrap_line(vec![], 20).len(), 1);
    /// ```
    pub fn wrap_line(&self, spans: Vec<Span<'static>>, width: usize) -> Vec<Line<'static>> {
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
