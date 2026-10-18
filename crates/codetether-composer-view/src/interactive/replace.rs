//! Application history and suggestions after character-range replacement.
use std::ops::Range;
impl super::Composer<'_> {
    /// Replace a character range, then detach history and refresh completions.
    pub fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        crate::InputEdit::new(self.input, self.cursor).replace_input_chars(range, replacement);
        self.history.history_index = None;
        self.refresh();
    }
}
