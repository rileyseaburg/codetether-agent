//! Application model decoding, including trimmed IDs and supplied names.
use crate::ModelInfo;
use serde_json::Value;
pub(super) fn decode(entry: &Value) -> Option<ModelInfo> {
    let id = match entry {
        Value::String(s) => s.trim().to_string(),
        Value::Object(_) => entry.get("id").and_then(Value::as_str)?.trim().to_string(),
        _ => return None,
    };
    if id.is_empty() {
        return None;
    }
    let name = entry
        .get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .unwrap_or(&id)
        .to_string();
    Some(ModelInfo {
        context_window: if id.contains("glm-5.3") || id.contains("glm-5.2") {
            1_000_000
        } else if id.contains("glm-5") {
            200_000
        } else {
            128_000
        },
        id,
        name,
        provider: "zai".to_string(),
        max_output_tokens: Some(128_000),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    })
}
