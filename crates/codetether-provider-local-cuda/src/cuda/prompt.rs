//! Local CUDA prompt operations.
use super::*;
impl<R: Runtime> LocalCudaProvider<R> {
    pub(super) fn strip_inline_think_blocks(raw: &str) -> String {
        codetether_provider_core::local::LocalCudaPrompt::strip_inline_think_blocks(raw)
    }

    pub(super) fn map_finish_reason(reason: Option<&str>) -> FinishReason {
        codetether_provider_core::local::LocalCudaPrompt::map_finish_reason(reason)
    }

    pub(super) fn to_prompts(messages: &[Message]) -> (String, String) {
        codetether_provider_core::local::LocalCudaPrompt::to_prompts(messages)
    }

    /// Format messages into a prompt string
    pub(super) fn format_messages(messages: &[Message]) -> String {
        codetether_provider_core::local::LocalCudaPrompt::format_messages(messages)
    }

    pub(super) fn content_to_string(parts: &[ContentPart]) -> String {
        codetether_provider_core::local::LocalCudaPrompt::content_to_string(parts)
    }
}
