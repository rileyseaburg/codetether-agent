//! compact body snippet.
use crate::*;
impl GeminiWebProtocol {
    /// Truncate diagnostics by character count and flatten line breaks.
    pub fn compact_body_snippet(raw: &str, max_chars: usize) -> String {
        raw.chars()
            .take(max_chars)
            .collect::<String>()
            .replace(['\n', '\r'], " ")
            .trim()
            .to_string()
    }
}
