//! Native provider request binding for the read-only live replay.
use codetether_agent::provider::{Message, ToolDefinition};
pub(super) struct Request {
    pub messages: Vec<Message>,
    pub system: String,
    pub tools: Vec<ToolDefinition>,
    pub model: String,
}
impl codetether_context_request::attempt::Host for Request {
    fn target(&self) -> codetether_context_request::attempt::Target<'_> {
        codetether_context_request::attempt::Target {
            system_prompt: &self.system,
            provider: "openai-codex",
            model: &self.model,
            tools: &self.tools,
            temperature: None,
        }
    }
    fn proactive(&self) -> Option<&Message> {
        None
    }
    async fn prepare(&mut self, _: &mut Vec<Message>) {}
    fn history(&self) -> &[Message] {
        &self.messages
    }
}
