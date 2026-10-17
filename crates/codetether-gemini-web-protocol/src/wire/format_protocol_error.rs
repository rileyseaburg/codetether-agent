//! format protocol error.
use crate::*;
impl GeminiWebProtocol {
    pub fn format_protocol_error(code: i64, model: &str, raw: &str) -> String {
        let req_id = Self::extract_protocol_request_id(raw)
            .map(|id| format!(" request_id={id}."))
            .unwrap_or_default();
        let snippet = Self::compact_body_snippet(raw, 240);

        match code {
            469 => format!(
                "Gemini Web backend rejected the request (protocol code 469) for model `{model}`.{req_id} \
                 This is usually a transient web-backend/model-route issue or account entitlement mismatch. \
                 Try again, or switch to `gemini-web-thinking` / `gemini-web-fast`. Payload snippet: {snippet}"
            ),
            _ => format!(
                "Gemini Web backend returned protocol status code {code} for model `{model}`.{req_id} \
                 Payload snippet: {snippet}"
            ),
        }
    }
}
