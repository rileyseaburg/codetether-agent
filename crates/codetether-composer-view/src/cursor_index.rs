//! Unicode scalar counts and character-to-byte indexing.
/// Count input characters.
///
/// # Arguments
/// * `input` - Text to count.
///
/// # Returns
/// Unicode scalar count.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_index::input_char_count("éx"), 2);
/// ```
pub fn input_char_count(input: &str) -> usize {
    input.chars().count()
}
/// Find the byte boundary for a character index, clamping past the end.
///
/// # Arguments
/// * `input` - Source text.
/// * `char_index` - Unicode scalar index.
///
/// # Returns
/// The byte boundary, clamped to the end of the text.
///
/// # Examples
/// ```
/// assert_eq!(codetether_composer_view::cursor_index::char_to_byte_index("éx", 1), 2);
/// ```
pub fn char_to_byte_index(input: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }
    input
        .char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or_else(|| input.len())
}
pub(super) fn char_at(input: &str, char_index: usize) -> Option<char> {
    input.chars().nth(char_index)
}
