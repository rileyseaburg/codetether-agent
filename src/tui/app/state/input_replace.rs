//! Application history and suggestions after character-range replacement.
use std::ops::Range;
impl super::AppState {
    pub fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        codetether_composer_view::InputEdit::new(&mut self.input, &mut self.input_cursor)
            .replace_input_chars(range, replacement);
        self.history_index = None;
        self.refresh_slash_suggestions();
    }
}
