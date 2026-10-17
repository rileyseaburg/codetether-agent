//! Provider compact body snippet.
use crate::*;
impl GeminiWebProvider {
    pub(crate) fn compact_body_snippet(raw: &str, max_chars: usize) -> String {
        codetether_gemini_web_protocol::GeminiWebProtocol::compact_body_snippet(raw, max_chars)
    }
}
