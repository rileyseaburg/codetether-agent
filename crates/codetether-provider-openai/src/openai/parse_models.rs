//! Parse a provider `/models` payload into [`ModelInfo`] records.

use serde_json::Value;

use codetether_provider_core::ModelInfo;

use super::OpenAIProvider;

impl OpenAIProvider {
    pub(super) fn parse_models_payload(payload: &Value, provider_name: &str) -> Vec<ModelInfo> {
        payload
            .get("data")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                if provider_name == "groq" && !super::groq_models::is_chat(entry) {
                    return None;
                }
                let mut model = Self::model_info_from_api_entry(entry, provider_name)?;
                if provider_name == "groq" && model.id.starts_with("groq/compound") {
                    model.supports_tools = false;
                }
                Some(model)
            })
            .collect()
    }
}

pub(super) fn value_to_usize(value: Option<&Value>) -> Option<usize> {
    value
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
}
