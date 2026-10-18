//! Provider finish reason to OpenAI wire label conversion.

/// Map a provider finish reason to its wire label.
/// # Arguments
/// reason is the provider completion reason.
/// # Returns
/// The corresponding OpenAI-compatible label.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::convert_finish_reason(codetether_provider_core::FinishReason::Stop), "stop");
/// ```
pub fn convert_finish_reason(reason: codetether_provider_core::FinishReason) -> &'static str {
    match reason {
        codetether_provider_core::FinishReason::Stop => "stop",
        codetether_provider_core::FinishReason::Length => "length",
        codetether_provider_core::FinishReason::ToolCalls => "tool_calls",
        codetether_provider_core::FinishReason::ContentFilter => "content_filter",
        codetether_provider_core::FinishReason::Error => "error",
    }
}
