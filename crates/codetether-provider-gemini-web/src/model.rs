//! Shared Gemini Web model routing.
use super::GeminiWebProvider;
impl GeminiWebProvider {
    pub(super) fn resolved_model(model: &str) -> String {
        codetether_gemini_web_protocol::GeminiWebProtocol::resolved_model(model)
    }
    #[cfg(test)]
    pub(super) fn mode_id(model: &str) -> &'static str {
        codetether_gemini_web_protocol::GeminiWebProtocol::mode_id(model)
    }
}
