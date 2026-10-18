//! Word-boundary policy for starting symbol mentions.

/// Whether an `@` typed at the cursor starts a mention.
///
/// Only a word-initial `@` opens the picker, so `user@host`, `@decorator`
/// mid-word, and git refs insert a literal `@` instead.
/// # Arguments
/// * `input` - Current composer text.
/// * `cursor` - Cursor position measured in Unicode characters.
/// # Returns
/// Whether the cursor is at the start or follows whitespace.
/// # Examples
/// ```
/// assert!(codetether_symbol_mention::starts_mention("fix ", 4));
/// assert!(!codetether_symbol_mention::starts_mention("user", 4));
/// ```
pub fn starts_mention(input: &str, cursor: usize) -> bool {
    if cursor == 0 {
        return true;
    }
    input
        .chars()
        .nth(cursor - 1)
        .is_some_and(char::is_whitespace)
}
