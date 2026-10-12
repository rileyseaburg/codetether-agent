//! Model-list decoding for string and object entries.
use crate::ModelInfo;
use serde_json::Value;

pub(crate) fn parse(value: &Value) -> Vec<ModelInfo> {
    value
        .get("data")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let id = entry.as_str().or_else(|| entry.get("id")?.as_str())?;
            let context_window = if id.contains("glm-5.2") || id.contains("glm-5.3") {
                1_000_000
            } else if id.contains("glm-5") {
                200_000
            } else {
                128_000
            };
            Some(ModelInfo {
                id: id.into(),
                name: id.into(),
                provider: "zai".into(),
                context_window,
                max_output_tokens: Some(128_000),
                supports_vision: id.contains("vision") || id.contains("vl"),
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
        })
        .collect()
}
