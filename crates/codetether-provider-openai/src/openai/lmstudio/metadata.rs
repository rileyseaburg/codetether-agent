//! Parse loaded inference instances without falling back to advertised limits.

use super::*;
use serde_json::Value;

pub(super) fn models(payload: &Value, provider: &str) -> Result<Vec<ModelInfo>> {
    let entries = payload
        .get("data")
        .and_then(Value::as_array)
        .context("LM Studio /api/v0/models response has no data array")?;
    Ok(entries
        .iter()
        .filter_map(|entry| {
            if entry.get("state").and_then(Value::as_str) != Some("loaded")
                || !matches!(
                    entry.get("type").and_then(Value::as_str),
                    Some("llm" | "vlm")
                )
            {
                return None;
            }
            let capacity = entry
                .get("loaded_context_length")
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .filter(|value| *value > 0)?;
            let mut model = OpenAIProvider::model_info_from_api_entry(entry, provider)?;
            model.context_window = capacity;
            // A safe client output ceiling, not an architectural output limit.
            model.max_output_tokens = Some(budget::output_ceiling(capacity));
            // Image token cost cannot be bounded from URL length; guard is text-only.
            model.supports_vision = false;
            Some(model)
        })
        .collect())
}
