//! Choose a whitespace or character boundary within display width.
use unicode_width::UnicodeWidthChar;

/// Take the longest prefix of `text` whose display width fits in `width`.
///
/// Prefers breaking after the last whitespace inside the fitting prefix.
/// When no whitespace is available (e.g. a long URL), falls back to a hard
/// char-boundary split at the last character that still fits.
///
/// # Arguments
///
/// * `text` — UTF-8 input, possibly wider than `width`.
/// * `width` — maximum display columns the returned `taken` may occupy.
/// * `at_start` — if `true`, leading whitespace is trimmed before measuring
///   so wrapped continuation rows don't begin with a space.
///
/// # Returns
///
/// Tuple `(taken, rest)` where `taken` fits in `width` columns and
/// `rest` is the remainder to wrap onto following rows. If the whole
/// input fits, `rest` is empty.
pub(crate) fn take_fit(text: &str, width: usize, at_start: bool) -> (String, String) {
    let trimmed = if at_start { text.trim_start() } else { text };
    let mut end_byte = 0usize;
    let mut last_ws_byte: Option<usize> = None;
    let mut w: usize = 0;
    for (i, ch) in trimmed.char_indices() {
        let cw = UnicodeWidthChar::width(ch).unwrap_or(0);
        if w + cw > width {
            break;
        }
        w += cw;
        end_byte = i + ch.len_utf8();
        if ch.is_whitespace() {
            last_ws_byte = Some(end_byte);
        }
    }
    if end_byte == trimmed.len() {
        return (trimmed.to_string(), String::new());
    }
    let split = last_ws_byte.unwrap_or(end_byte).max(1).min(trimmed.len());
    let taken = trimmed[..split].trim_end().to_string();
    let rest = trimmed[split..].to_string();
    (taken, rest)
}
