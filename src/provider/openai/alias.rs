//! Cerebras/Z.ai model-id aliasing for the OpenAI-compatible provider.

/// Map a logical model id to the slug a provider actually accepts.
///
/// Cerebras hosts Z.ai's GLM as `zai-glm-4.7` while the rest of the catalog
/// calls it `glm-4.7`. Routing the bare id to Cerebras yields `model_not_found`
/// (400), which surfaces as `stream faulted (transient=false)`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::openai::alias::normalize_model_id;
///
/// assert_eq!(normalize_model_id("cerebras", "glm-4.7"), "zai-glm-4.7");
/// assert_eq!(normalize_model_id("cerebras", "zai-glm-4.7"), "zai-glm-4.7");
/// assert_eq!(normalize_model_id("cerebras", "gpt-oss-120b"), "gpt-oss-120b");
/// assert_eq!(normalize_model_id("openai", "glm-4.7"), "glm-4.7");
/// ```
pub fn normalize_model_id<'a>(provider_name: &str, model: &'a str) -> std::borrow::Cow<'a, str> {
    if provider_name == "cerebras" && model.starts_with("glm-") && !model.starts_with("zai-") {
        return std::borrow::Cow::Owned(format!("zai-{model}"));
    }
    std::borrow::Cow::Borrowed(model)
}
