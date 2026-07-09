//! Raw placeholder expansion for sidecar paste recovery.
//!
//! Unlike [`super::pasted_text::expand_paste_placeholders`], this
//! expansion inserts the sidecar content verbatim — no delimiter
//! banners — so a base64 payload split across several sidecar
//! entries reassembles into one contiguous run.

use super::pasted_text::PendingTextPaste;

/// Replace each `[Pasted text #N: …]` placeholder with its raw content.
pub(super) fn expand_placeholders_raw(prompt: &str, pastes: &[PendingTextPaste]) -> String {
    let mut out = prompt.to_string();
    for paste in pastes {
        let ph = paste.placeholder();
        if out.contains(&ph) {
            out = out.replacen(&ph, &paste.content, 1);
        }
    }
    out
}
