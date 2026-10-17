//! Local-provider map finish reason.
use super::*;
impl LocalCudaPrompt {
    /// Map the local runtime finish marker to the common provider enum.
    pub fn map_finish_reason(reason: Option<&str>) -> FinishReason {
        match reason {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("error") => FinishReason::Error,
            _ => FinishReason::Stop,
        }
    }
}
