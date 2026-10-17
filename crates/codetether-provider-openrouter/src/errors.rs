//! Extended error envelopes, including embedded successful-HTTP errors.
use crate::*;
impl OpenRouterProvider {
    pub(crate) fn parse_error_body(text: &str) -> Option<String> {
        let err = serde_json::from_str::<OpenRouterError>(text).ok()?;
        let mut message = format!("OpenRouter API error: {}", err.error.message);
        if let Some(code) = err.error.code {
            message.push_str(&format!(" (code: {code})"));
        }
        if let Some(detail) = err.error.metadata.as_ref().and_then(error_detail::extract) {
            message.push_str(&format!(": {detail}"));
        }
        Some(message)
    }
}
