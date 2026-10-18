//! Whitespace-delimited word movement to the right.
use super::cursor_index::{char_at, input_char_count};
use super::cursor_move::clamp_input_cursor;
/// Compute the cursor after skipping whitespace and the adjacent word.
///
/// # Arguments
/// * `input` - Current text.
/// * `cursor` - Requested character index.
///
/// # Returns
/// The cursor after stepping right across whitespace and one word.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_word_right::move_cursor_word_right("one two", 0), 3);
/// ```
pub fn move_cursor_word_right(input: &str, mut cursor: usize) -> usize {
    cursor = clamp_input_cursor(input, cursor);
    while cursor < input_char_count(input)
        && char_at(input, cursor).is_some_and(char::is_whitespace)
    {
        cursor += 1;
    }
    while cursor < input_char_count(input)
        && char_at(input, cursor).is_some_and(|c| !c.is_whitespace())
    {
        cursor += 1;
    }

    cursor
}
