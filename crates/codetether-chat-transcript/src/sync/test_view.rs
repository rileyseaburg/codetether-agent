//! In-memory transcript sink for projection-focused synchronization tests.
use super::{Metrics, View};
use codetether_chat_model::ChatMessage;
use codetether_provider_core::Message;
use codetether_telemetry::{CompletionSummary, RequestTimingState, ToolTimingState};

#[derive(Default)]
pub(super) struct ProjectionView {
    pub messages: Vec<ChatMessage>,
    request: RequestTimingState,
    tool: ToolTimingState,
    completion: CompletionSummary,
}

impl View for ProjectionView {
    fn replace_messages(&mut self, messages: Vec<ChatMessage>) {
        self.messages = messages;
    }
    fn reset_history(&mut self, _: String, _: &[Message], _: usize, _: bool) {}
    fn set_history_before(&mut self, _: usize) {}
    fn clear_cache(&mut self) {}
    fn metrics(&mut self) -> Metrics<'_> {
        Metrics {
            request: &mut self.request,
            tool: &mut self.tool,
            model: &mut self.completion.model,
            latency: &mut self.completion.latency_ms,
            prompt_tokens: &mut self.completion.prompt_tokens,
            output_tokens: &mut self.completion.output_tokens,
        }
    }
    fn reset_tool_preview_scroll(&mut self) {}
    fn set_tool_preview_max_scroll(&mut self, _: usize) {}
    fn scroll_to_bottom(&mut self) {}
}
