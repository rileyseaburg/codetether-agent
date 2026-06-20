//! Parse a provider `/models` payload into [`ModelInfo`] records.

use serde_json::Value;

use crate::provider::ModelInfo;

use super::OpenAIProvider;

impl OpenAIProvider {
    pub(super) fn parse_models_payload(payload: &Value, provider_name: &str) -> Vec<ModelInfo> {
        payload
            .get("data")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| Self::model_info_from_api_entry(entry, provider_name))
            .collect()
    }
}

pub(super) fn value_to_usize(value: Option<&Value>) -> Option<usize> {
    value
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
}
