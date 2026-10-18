//! Reassemble paste placeholders as verbatim content.
use crate::PendingTextPaste;

/// Replace each `[Pasted text #N: …]` placeholder with its raw content.
///
/// # Arguments
/// * `prompt` — Input text containing placeholders.
/// * `pastes` — Sidecar entries available for expansion.
///
/// # Returns
/// Prompt text with each matching placeholder replaced by raw content once.
///
/// # Examples
/// ```
/// assert_eq!(codetether_paste_buffer::expand_placeholders_raw("hello", &[]), "hello");
/// ```
pub fn expand_placeholders_raw(prompt: &str, pastes: &[PendingTextPaste]) -> String {
    let mut out = prompt.to_string();
    for paste in pastes {
        let ph = paste.placeholder();
        if out.contains(&ph) {
            out = out.replacen(&ph, &paste.content, 1);
        }
    }
    out
}
