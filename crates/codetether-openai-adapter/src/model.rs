//! OpenAI-compatible model conversion.

/// Resolve the supported provider-name alias.
/// # Arguments
/// provider is the requested provider name.
/// # Returns
/// zai for zhipuai, otherwise the original name.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::canonicalize_provider_name("zhipuai"), "zai");
/// ```
pub fn canonicalize_provider_name(provider: &str) -> &str {
    if provider == "zhipuai" {
        "zai"
    } else {
        provider
    }
}

/// Normalize a colon-separated model reference.
/// # Arguments
/// model is the incoming model identifier.
/// # Returns
/// Trimmed identifier, converting provider:model to provider/model where applicable.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::normalize_model_reference("a:b"), "a/b");
/// ```
pub fn normalize_model_reference(model: &str) -> String {
    let trimmed = model.trim();
    if let Some((provider, model_id)) = trimmed.split_once(':')
        && !provider.is_empty()
        && !model_id.is_empty()
        && !trimmed.contains('/')
    {
        return format!("{provider}/{model_id}");
    }
    trimmed.to_string()
}

/// Build a provider-qualified model identifier.
/// # Arguments
/// provider and model_id identify the response model.
/// # Returns
/// Qualified model identifier without a duplicate provider prefix.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::make_openai_model_id("a", "/b"), "a/b");
/// ```
pub fn make_openai_model_id(provider: &str, model_id: &str) -> String {
    let provider = canonicalize_provider_name(provider);
    let trimmed = model_id.trim_start_matches('/');
    if trimmed.starts_with(&format!("{provider}/")) {
        trimmed.to_string()
    } else {
        format!("{provider}/{trimmed}")
    }
}
