//! Cursor movement and visibility over the borrowed composer state.
use super::Composer;
impl Composer<'_> {
    /// Clamp the cursor to the current input length.
    pub fn clamp_input_cursor(&mut self) {
        *self.cursor = crate::cursor_move::clamp_input_cursor(self.input, *self.cursor);
    }
    /// Keep the cursor in the visible horizontal range.
    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        *self.scroll = crate::cursor_visibility::ensure_input_cursor_visible(
            *self.cursor,
            *self.scroll,
            visible_width,
        );
    }
    /// Move left by one Unicode scalar.
    pub fn move_cursor_left(&mut self) {
        *self.cursor = crate::cursor_move::move_cursor_left(*self.cursor);
    }
    /// Move right by one Unicode scalar.
    pub fn move_cursor_right(&mut self) {
        *self.cursor = crate::cursor_move::move_cursor_right(self.input, *self.cursor);
    }
    /// Move to the preceding word boundary.
    pub fn move_cursor_word_left(&mut self) {
        *self.cursor = crate::cursor_word_left::move_cursor_word_left(self.input, *self.cursor);
    }
    /// Move to the following word boundary.
    pub fn move_cursor_word_right(&mut self) {
        *self.cursor = crate::cursor_word_right::move_cursor_word_right(self.input, *self.cursor);
    }
    /// Move to the start of the input.
    pub fn move_cursor_home(&mut self) {
        *self.cursor = crate::cursor_move::move_cursor_home();
    }
    /// Move to the end of the input.
    pub fn move_cursor_end(&mut self) {
        *self.cursor = crate::cursor_move::move_cursor_end(self.input);
    }
}
