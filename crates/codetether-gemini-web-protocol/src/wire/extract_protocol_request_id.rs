//! extract protocol request id.
use crate::*;
impl GeminiWebProtocol {
    /// Best-effort extraction of Gemini request id tokens like `r_52bc...`
    /// from protocol frames for diagnostics.
    pub fn extract_protocol_request_id(raw: &str) -> Option<String> {
        static RE_REQ_ID: OnceLock<Regex> = OnceLock::new();
        let re = RE_REQ_ID.get_or_init(|| Regex::new(r"(r_[A-Za-z0-9]+)").unwrap());
        re.captures(raw)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
    }
}
