//! Provider build freq.
use crate::*;
impl GeminiWebProvider {
    /// Build the `f.req` JSON payload — a two-element outer array whose
    /// second slot is a JSON-encoded 69-element inner array.
    ///
    /// Index layout matches the Python GemChat reference implementation.
    pub(crate) fn build_freq(prompt: &str) -> String {
        codetether_gemini_web_protocol::GeminiWebProtocol::build_freq(prompt)
    }
}
