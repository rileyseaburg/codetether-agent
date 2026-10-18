//! Empty-section formatting for a working-tree report.

pub(super) fn fallback(text: &str) -> &str {
    if text.trim().is_empty() {
        "(none)"
    } else {
        text
    }
}
