//! Native tool-channel capability selection for the resolved provider and model.

/// Determine whether the resolved provider/model has a native tool-call channel.
/// # Arguments
/// `provider` is the provider ID; `model` is its model ID.
/// # Returns
/// False for local CUDA aliases and unsupported OpenRouter models.
/// # Examples
/// ```
/// assert!(!codetether_request_preparation::model_supports_tools("local_cuda", "model"));
/// ```
pub fn model_supports_tools(provider: &str, model: &str) -> bool {
    // local_cuda has no native tool-call channel, so the caller injects a
    // textual `<tool_call>` contract and parses it back via
    // `markup::normalize_textual_tool_calls`.
    if matches!(provider, "local-cuda" | "local_cuda" | "localcuda") {
        return false;
    }
    provider != "openrouter"
        || codetether_provider_openrouter::model_capabilities::supports_tools(model)
}
