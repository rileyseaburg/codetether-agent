//! Owned composer cursor operations.
impl crate::ComposerState {
    /// Count Unicode scalar values in the input.
    pub fn input_char_count(&self) -> usize {
        crate::cursor_index::input_char_count(&self.input)
    }
    /// Convert a character position to an input byte index.
    pub fn char_to_byte_index(&self, char_index: usize) -> usize {
        crate::cursor_index::char_to_byte_index(&self.input, char_index)
    }
    /// Clamp the cursor to the current input length.
    pub fn clamp_input_cursor(&mut self) {
        self.borrow().clamp_input_cursor();
    }
    /// Scroll horizontally to keep the cursor visible.
    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        self.borrow().ensure_input_cursor_visible(visible_width);
    }
    /// Move the cursor one character left.
    pub fn move_cursor_left(&mut self) {
        self.borrow().move_cursor_left();
    }
    /// Move the cursor one character right.
    pub fn move_cursor_right(&mut self) {
        self.borrow().move_cursor_right();
    }
    /// Move the cursor to the preceding word boundary.
    pub fn move_cursor_word_left(&mut self) {
        self.borrow().move_cursor_word_left();
    }
    /// Move the cursor to the following word boundary.
    pub fn move_cursor_word_right(&mut self) {
        self.borrow().move_cursor_word_right();
    }
    /// Move the cursor to the start of input.
    pub fn move_cursor_home(&mut self) {
        self.borrow().move_cursor_home();
    }
    /// Move the cursor to the end of input.
    pub fn move_cursor_end(&mut self) {
        self.borrow().move_cursor_end();
    }
}
