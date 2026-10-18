//! Filter line breaks before feeding single-line destinations.

/// Iterate non-newline characters and feed them to `f`.
pub(super) fn paste_chars_no_newlines(text: &str, mut f: impl FnMut(char)) {
    for ch in text.chars().filter(|ch| *ch != '\n') {
        f(ch);
    }
}
