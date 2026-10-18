//! Owned composer edit operations.
impl crate::ComposerState {
    /// Insert a character and update history and completion state.
    pub fn insert_char(&mut self, c: char) {
        self.borrow().insert_char(c);
    }
    /// Insert text and update history and completion state.
    pub fn insert_text(&mut self, text: &str) {
        self.borrow().insert_text(text);
    }
    /// Delete the preceding character and refresh completion state.
    pub fn delete_backspace(&mut self) {
        self.borrow().delete_backspace();
    }
    /// Delete the following character and refresh completion state.
    pub fn delete_forward(&mut self) {
        self.borrow().delete_forward();
    }
    /// Clear input and reset its cursor, mode, completion and history selection.
    pub fn clear_input(&mut self) {
        self.borrow().clear_input();
    }
    /// Replace a character range using the shared editing workflow.
    pub fn replace_input_chars(&mut self, range: ::std::ops::Range<usize>, replacement: &str) {
        self.borrow().replace_input_chars(range, replacement);
    }
}
