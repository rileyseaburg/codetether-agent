//! Model-specific OpenRouter capability overrides.
//!
//! OpenRouter is a router, not a model family: some catalog entries are
//! classifiers and have no endpoint that accepts function tools. Advertising
//! the provider's full catalog to those models yields HTTP 404 before inference.

/// Returns whether `model` accepts OpenAI-compatible function tools.
///
/// Safety classifiers emit labels, not agent actions. OpenRouter confirms this
/// with `No endpoints found that support tool use` for
/// `nvidia/nemotron-3.5-content-safety:free`.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openrouter::model_capabilities::supports_tools;
///
/// assert!(!supports_tools("nvidia/nemotron-3.5-content-safety:free"));
/// assert!(supports_tools("google/gemma-4-26b-a4b-it:free"));
/// ```
pub fn supports_tools(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    !normalized.contains("content-safety")
        && !normalized.contains("content_safety")
        && !normalized.contains("/moderation")
}

/// Uses OpenRouter's catalog metadata when present, falling back to the known
/// model override for older payloads that omit `supported_parameters`.
pub(super) fn catalog_supports_tools(model: &str, parameters: &[String]) -> bool {
    supports_tools(model) && (parameters.is_empty() || parameters.iter().any(|p| p == "tools"))
}
