//! Worker model parse policy.
use super::*;
use codetether_provider_core::routing::parse_model_string;

pub(super) fn parse_model(model: &str) -> (Option<String>, String) {
    let (provider, model_id) = parse_model_string(model);
    let provider = provider.map(|value| if value == "zhipuai" { "zai" } else { value });
    (provider.map(ToString::to_string), model_id.to_string())
}
