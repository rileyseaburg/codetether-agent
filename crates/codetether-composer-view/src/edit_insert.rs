//! Unicode-aware insertion.
use crate::{InputEdit, cursor_index::char_to_byte_index, cursor_move::clamp_input_cursor};
impl InputEdit<'_> {
    /// Apply the edit and return whether text changed.
    pub fn insert_char(&mut self, c: char) -> bool {
        *self.cursor = clamp_input_cursor(self.text, *self.cursor);
        let byte_index = char_to_byte_index(self.text, *self.cursor);
        self.text.insert(byte_index, c);
        *self.cursor += 1;

        true
    }
    /// Apply the edit and return whether text changed.
    pub fn insert_text(&mut self, text: &str) -> bool {
        if text.is_empty() {
            return false;
        }
        *self.cursor = clamp_input_cursor(self.text, *self.cursor);
        let byte_index = char_to_byte_index(self.text, *self.cursor);
        self.text.insert_str(byte_index, text);
        *self.cursor += text.chars().count();

        true
    }
}
