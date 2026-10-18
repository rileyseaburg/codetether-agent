//! Deletion at character boundaries.
use crate::{
    InputEdit,
    cursor_index::{char_to_byte_index, input_char_count},
    cursor_move::clamp_input_cursor,
};
impl InputEdit<'_> {
    /// Apply the edit and return whether text changed.
    pub fn delete_backspace(&mut self) -> bool {
        *self.cursor = clamp_input_cursor(self.text, *self.cursor);
        if *self.cursor == 0 {
            return false;
        }
        let start = char_to_byte_index(self.text, *self.cursor - 1);
        let end = char_to_byte_index(self.text, *self.cursor);
        self.text.replace_range(start..end, "");
        *self.cursor -= 1;

        true
    }
    /// Apply the edit and return whether text changed.
    pub fn delete_forward(&mut self) -> bool {
        *self.cursor = clamp_input_cursor(self.text, *self.cursor);
        if *self.cursor >= input_char_count(self.text) {
            return false;
        }
        let start = char_to_byte_index(self.text, *self.cursor);
        let end = char_to_byte_index(self.text, *self.cursor + 1);
        self.text.replace_range(start..end, "");

        true
    }
}
