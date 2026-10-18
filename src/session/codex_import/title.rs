use super::payloads::CodexReasoningPayload;
pub(crate) use codetether_codex_discovery::derive_title_from_text;
use serde_json::Value;

pub(crate) fn extract_reasoning_text(payload: &CodexReasoningPayload) -> Option<String> {
    if let Some(content) = payload
        .content
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return Some(content.to_string());
    }

    let parts = payload
        .summary
        .iter()
        .filter_map(summary_value_to_text)
        .collect::<Vec<_>>();
    (!parts.is_empty()).then(|| parts.join("\n"))
}

fn summary_value_to_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => {
            let text = text.trim();
            (!text.is_empty()).then(|| text.to_string())
        }
        Value::Object(map) => map
            .get("text")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_string),
        _ => None,
    }
}
