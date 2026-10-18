//! Retry missing calls only for the established provider/model cases.
use crate::assistant_claims_imminent_tool_use;
use codetether_provider_core::ToolDefinition;

/// Apply the established provider has flaky native tool calling policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert!(codetether_provider_policy::provider_has_flaky_native_tool_calling("minimax", "model"));
/// ```
pub fn provider_has_flaky_native_tool_calling(provider_name: &str, model: &str) -> bool {
    let provider = provider_name.to_ascii_lowercase();
    let model = model.to_ascii_lowercase();
    provider == "minimax"
        || provider == "minimax-credits"
        || model.contains("minimax")
        || model.contains("m2.5")
}

/// Apply the established should retry missing native tool call policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert!(!codetether_provider_policy::should_retry_missing_native_tool_call("openai", "model", 0, &[], "", false, 1));
/// ```
pub fn should_retry_missing_native_tool_call(
    provider_name: &str,
    model: &str,
    retry_count: u8,
    tool_definitions: &[ToolDefinition],
    assistant_text: &str,
    has_tool_calls: bool,
    native_tool_promise_retry_max_retries: u8,
) -> bool {
    if retry_count >= native_tool_promise_retry_max_retries
        || has_tool_calls
        || tool_definitions.is_empty()
        || !provider_has_flaky_native_tool_calling(provider_name, model)
    {
        return false;
    }

    assistant_claims_imminent_tool_use(assistant_text, tool_definitions)
}
