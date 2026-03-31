use super::payloads::CodexReasoningPayload;
use serde_json::Value;

pub(crate) fn derive_title_from_text(text: &str) -> Option<String> {
    let first_line = text.lines().map(str::trim).find(|line| !line.is_empty())?;
    let mut title = first_line.to_string();
    if title.len() > 60 {
        title.truncate(57);
        title.push_str("...");
    }
    Some(title)
}

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
