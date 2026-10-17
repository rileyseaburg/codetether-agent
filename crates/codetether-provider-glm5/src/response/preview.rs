//! UTF-8-safe error previews.
use crate::*;
impl Glm5Provider {
    pub(crate) fn preview_text(text: &str, max_chars: usize) -> &str {
        if max_chars == 0 {
            return "";
        }
        if let Some((idx, _)) = text.char_indices().nth(max_chars) {
            &text[..idx]
        } else {
            text
        }
    }
}
