//! Native completion metric and usage message fields.
use super::Native;
use codetether_chat_model::ChatMessage;
impl codetether_chat_events::UsageView for Native<'_> {
    fn set_model(&mut self, model: String) {
        self.0.state.last_completion_model = Some(model);
    }
    fn set_latency(&mut self, duration_ms: u64) {
        self.0.state.last_completion_latency_ms = Some(duration_ms);
    }
    fn set_prompt_tokens(&mut self, tokens: usize) {
        self.0.state.last_completion_prompt_tokens = Some(tokens);
    }
    fn set_output_tokens(&mut self, tokens: usize) {
        self.0.state.last_completion_output_tokens = Some(tokens);
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.0.state.messages
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
}
