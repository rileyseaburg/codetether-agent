//! Interpret the server stop reason and tool-call precedence.
use codetether_provider_core::FinishReason;
pub(super) fn finish(reason: &Option<String>, has_calls: bool) -> FinishReason {
    if has_calls {
        return FinishReason::ToolCalls;
    }
    match reason.as_deref() {
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCalls,
        _ => FinishReason::Stop,
    }
}
