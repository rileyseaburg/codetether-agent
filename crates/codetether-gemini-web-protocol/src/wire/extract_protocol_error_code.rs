//! extract protocol error code.
use crate::*;
impl GeminiWebProtocol {
    /// Extract Gemini protocol-level error code from SSE-like body lines.
    ///
    /// Looks for events like: `[["e",5,null,null,469]]`
    pub fn extract_protocol_error_code(raw: &str) -> Option<i64> {
        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('[') {
                continue;
            }
            let Ok(events_val) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            let Some(events) = events_val.as_array() else {
                continue;
            };
            for event in events {
                let Some(ev) = event.as_array() else { continue };
                let Some(kind) = ev.first().and_then(Value::as_str) else {
                    continue;
                };
                if kind != "e" {
                    continue;
                }
                if let Some(code) = ev.get(4).and_then(Value::as_i64) {
                    return Some(code);
                }
                if let Some(code) = ev.last().and_then(Value::as_i64) {
                    return Some(code);
                }
            }
        }
        None
    }
}
