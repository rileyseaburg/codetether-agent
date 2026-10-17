//! Provider extract protocol request id.
use crate::*;
impl GeminiWebProvider {
    /// Best-effort extraction of Gemini request id tokens like `r_52bc...`
    /// from protocol frames for diagnostics.
    pub(crate) fn extract_protocol_request_id(raw: &str) -> Option<String> {
        codetether_gemini_web_protocol::GeminiWebProtocol::extract_protocol_request_id(raw)
    }
}
