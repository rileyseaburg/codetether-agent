//! Whitespace-delimited word movement to the left.
use super::cursor_index::char_at;
use super::cursor_move::clamp_input_cursor;
/// Compute the cursor after skipping whitespace and the adjacent word.
///
/// # Arguments
/// * `input` - Current text.
/// * `cursor` - Requested character index.
///
/// # Returns
/// The cursor after stepping left across whitespace and one word.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_word_left::move_cursor_word_left("one two", 7), 4);
/// ```
pub fn move_cursor_word_left(input: &str, mut cursor: usize) -> usize {
    cursor = clamp_input_cursor(input, cursor);
    while cursor > 0 && char_at(input, cursor - 1).is_some_and(char::is_whitespace) {
        cursor -= 1;
    }
    while cursor > 0 && char_at(input, cursor - 1).is_some_and(|c| !c.is_whitespace()) {
        cursor -= 1;
    }

    cursor
}
