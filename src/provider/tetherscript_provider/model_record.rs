//! Test fixtures for TetherScript model-list normalization.

pub(super) fn without_provider(id: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": id,
        "context_window": 128000,
        "max_output_tokens": 8192,
        "supports_vision": false,
        "supports_tools": true,
        "supports_streaming": true,
        "input_cost_per_million": null,
        "output_cost_per_million": null
    })
}

pub(super) fn with_provider(id: &str, provider: &str) -> serde_json::Value {
    let mut model = without_provider(id);
    model["provider"] = serde_json::Value::String(provider.to_string());
    model
}
