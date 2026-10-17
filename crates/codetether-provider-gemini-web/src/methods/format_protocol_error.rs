//! Provider format protocol error.
use crate::*;
impl GeminiWebProvider {
    pub(crate) fn format_protocol_error(code: i64, model: &str, raw: &str) -> String {
        codetether_gemini_web_protocol::GeminiWebProtocol::format_protocol_error(code, model, raw)
    }
}
