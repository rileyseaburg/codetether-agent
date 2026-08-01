//! Sentence-boundary detection for reasoning/answer splitting.

/// Returns the byte index just past the first sentence terminator.
///
/// Only `.`, `!`, and `?` terminate a sentence. A terminator followed by an
/// ASCII digit is treated as part of a number (for example `3.5`) rather than a
/// boundary, so version strings do not split narration mid-thought.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::reasoning::sentence_end;
///
/// assert_eq!(sentence_end("One. Two."), Some(4));
/// assert_eq!(sentence_end("version 3.5 ships"), None);
/// assert_eq!(sentence_end("no terminator"), None);
/// ```
pub fn sentence_end(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if !matches!(byte, b'.' | b'!' | b'?') {
            continue;
        }
        if bytes.get(index + 1).is_some_and(u8::is_ascii_digit) {
            continue;
        }
        let end = index + 1;
        if text.is_char_boundary(end) {
            return Some(end);
        }
    }
    None
}
