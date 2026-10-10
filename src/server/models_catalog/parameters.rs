//! OpenRouter-style supported parameter names.

pub(crate) fn supported_parameters(model: &crate::provider::ModelInfo) -> Vec<String> {
    let mut params = base_parameters();
    if model.supports_tools {
        params.extend(["tools", "tool_choice"]);
    }
    params.into_iter().map(str::to_string).collect()
}

fn base_parameters() -> Vec<&'static str> {
    vec!["max_tokens", "temperature", "top_p", "stop"]
}

/// Base parameters plus `reasoning_effort`/`service_tier` when supported.
pub(crate) fn with_options(
    model: &crate::provider::ModelInfo,
    options: &super::options::ModelOptions,
) -> Vec<String> {
    let mut params = supported_parameters(model);
    if !options.reasoning_efforts.is_empty() {
        params.push("reasoning_effort".to_string());
    }
    if !options.service_tiers.is_empty() {
        params.push("service_tier".to_string());
    }
    params
}
