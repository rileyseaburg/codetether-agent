//! Expand visible paste placeholders into delimited prompt content.
use crate::PendingTextPaste;

/// Replace each `[Pasted text #N: …]` placeholder in `prompt` with
/// the corresponding sidecar content wrapped in delimiters that the
/// agent can recognise. Pastes whose placeholder no longer appears
/// in the prompt are dropped silently — the user must have edited
/// them out before submitting.
///
/// # Arguments
/// * `prompt` — Input text containing placeholders.
/// * `pastes` — Sidecar entries available for expansion.
///
/// # Returns
/// Prompt text with each matching placeholder expanded once.
///
/// # Examples
/// ```
/// assert_eq!(codetether_paste_buffer::expand_paste_placeholders("hello", &[]), "hello");
/// ```
pub fn expand_paste_placeholders(prompt: &str, pastes: &[PendingTextPaste]) -> String {
    let mut out = prompt.to_string();
    for paste in pastes {
        let ph = paste.placeholder();
        if !out.contains(&ph) {
            continue;
        }
        let expansion = format!(
            "\n\n--- Begin pasted text #{id} ({lines} lines, {bytes} bytes) ---\n{content}\n--- End pasted text #{id} ---",
            id = paste.id,
            lines = paste.lines(),
            bytes = paste.bytes(),
            content = paste.content,
        );
        out = out.replacen(&ph, &expansion, 1);
    }
    out
}
