//! Convert a single `/models` entry object into a [`ModelInfo`].

use serde_json::Value;

use crate::provider::ModelInfo;

use super::model_caps::{detect_vision, flag, price};
use super::parse_models::value_to_usize;
use super::OpenAIProvider;

impl OpenAIProvider {
    pub(super) fn model_info_from_api_entry(
        entry: &Value,
        provider_name: &str,
    ) -> Option<ModelInfo> {
        let id = match entry {
            Value::String(id) => id.trim(),
            Value::Object(_) => entry.get("id").and_then(Value::as_str)?.trim(),
            _ => return None,
        };
        if id.is_empty() {
            return None;
        }

        let name = entry
            .get("name")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or(id);

        Some(ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: provider_name.to_string(),
            context_window: value_to_usize(
                entry
                    .pointer("/limits/max_context_window_tokens")
                    .or_else(|| entry.get("context_window")),
            )
            .unwrap_or(128_000),
            max_output_tokens: value_to_usize(
                entry
                    .pointer("/limits/max_output_tokens")
                    .or_else(|| entry.get("max_output_tokens")),
            ),
            supports_vision: detect_vision(entry),
            supports_tools: flag(entry, "supports_tools"),
            supports_streaming: flag(entry, "supports_streaming"),
            input_cost_per_million: price(entry, "input_cost_per_million"),
            output_cost_per_million: price(entry, "output_cost_per_million"),
        })
    }
}
