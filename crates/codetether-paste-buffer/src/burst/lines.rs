pub(super) fn has_prior_newline(input: &str, cursor: usize) -> bool {
    input.chars().take(cursor).any(|c| c == '\n')
}
/// Count characters since the latest newline before the cursor.
/// # Arguments
/// * `input` - Composer text.
/// * `cursor` - Cursor position in characters.
/// # Returns
/// Character count on the current line, bounded by the available input.
/// # Examples
/// ```
/// assert_eq!(codetether_paste_buffer::burst::current_line_len("one\ntwo", 7), 3);
/// ```

pub fn current_line_len(input: &str, cursor: usize) -> usize {
    let mut len = 0;
    for c in input.chars().take(cursor) {
        if c == '\n' {
            len = 0;
        } else {
            len += 1;
        }
    }
    len
}
