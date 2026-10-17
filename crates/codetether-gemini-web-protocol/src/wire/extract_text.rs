//! extract text.
use crate::*;
impl GeminiWebProtocol {
    /// Walk streaming lines and return the latest nonempty answer text.
    ///
    /// Wire format: each line is a JSON array of events:
    ///   `[["wrb.fr", key_or_null, inner_json_str, ...], ...]`
    /// The inner JSON has the response text at `inner[4][0][1][0]`.
    /// Gemini usually sends cumulative chunks, but may replace a draft.
    pub fn extract_text(raw: &str) -> String {
        response_text::latest(raw)
    }
}
