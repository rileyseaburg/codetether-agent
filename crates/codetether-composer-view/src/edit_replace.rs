//! Character-range replacement within composer text.
use crate::{
    InputEdit,
    cursor_index::{char_to_byte_index, input_char_count},
    cursor_move::clamp_input_cursor,
};
use std::ops::Range;
impl InputEdit<'_> {
    /// Replace a clamped character range and position the cursor after its text.
    ///
    /// # Panics
    /// Panics for an inverted character range or when the resulting cursor
    /// arithmetic overflows, preserving the input operation's range contract.
    ///
    /// # Examples
    /// ```
    /// let mut text = "hello".to_string();
    /// let mut cursor = 5;
    /// codetether_composer_view::InputEdit::new(&mut text, &mut cursor)
    ///     .replace_input_chars(0..1, "H");
    /// assert_eq!(text, "Hello");
    /// ```
    pub fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        *self.cursor = clamp_input_cursor(self.text, *self.cursor);
        let start = char_to_byte_index(self.text, range.start.min(input_char_count(self.text)));
        let end = char_to_byte_index(self.text, range.end.min(input_char_count(self.text)));
        self.text.replace_range(start..end, replacement);
        *self.cursor = range.start + replacement.chars().count();
    }
}
