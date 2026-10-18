//! Provider extract protocol error code.
use crate::*;
impl GeminiWebProvider {
    /// Extract Gemini protocol-level error code from SSE-like body lines.
    ///
    /// Looks for events like: `[["e",5,null,null,469]]`
    #[cfg(test)]
    pub(crate) fn extract_protocol_error_code(raw: &str) -> Option<i64> {
        codetether_gemini_web_protocol::GeminiWebProtocol::extract_protocol_error_code(raw)
    }
}
