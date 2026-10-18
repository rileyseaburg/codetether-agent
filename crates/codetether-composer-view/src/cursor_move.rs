//! Character-step, clamp and endpoint navigation.
use super::cursor_index::input_char_count;
/// Compute the cursor after clamp input cursor.
///
/// # Arguments
/// * `input` - Current text.
/// * `cursor` - Requested character index.
///
/// # Returns
/// The cursor clamped to the character count.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_move::clamp_input_cursor("hi", 9), 2);
/// ```
pub fn clamp_input_cursor(input: &str, cursor: usize) -> usize {
    cursor.min(input_char_count(input))
}
/// Compute the cursor after move cursor left.
///
/// # Arguments
/// * `cursor` - Current character index.
///
/// # Returns
/// The previous character index, stopping at zero.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_move::move_cursor_left(0), 0);
/// ```
pub fn move_cursor_left(mut cursor: usize) -> usize {
    if cursor > 0 {
        cursor -= 1;
    }

    cursor
}
/// Compute the cursor after move cursor right.
///
/// # Arguments
/// * `input` - Current text.
/// * `cursor` - Current character index.
///
/// # Returns
/// The next character index when available.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_move::move_cursor_right("hi", 0), 1);
/// ```
pub fn move_cursor_right(input: &str, mut cursor: usize) -> usize {
    if cursor < input_char_count(input) {
        cursor += 1;
    }

    cursor
}
/// Compute the cursor after move cursor home.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// The first character index.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_move::move_cursor_home(), 0);
/// ```
pub fn move_cursor_home() -> usize {
    0
}
/// Compute the cursor after move cursor end.
///
/// # Arguments
/// * `input` - Current text.
///
/// # Returns
/// The character index after the last character.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_move::move_cursor_end("éx"), 2);
/// ```
pub fn move_cursor_end(input: &str) -> usize {
    input_char_count(input)
}
